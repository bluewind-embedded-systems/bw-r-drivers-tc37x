// CanNode trait impl to be moved on a separate file (annabo)
pub trait CanNode {
    fn node_id(&self) -> u8;
    fn is_enabled(&self) -> bool;
    fn is_suspended(&self) -> bool;

    fn enable_module(&self); 
    fn disable_module(&self);
    fn reset_module(&self);
    //fn set_clock_source(&self, clock_select: ClockSelect, clock_source: ClockSource); 
}

use tc37x_pac::can0::Can0;
use tc37x_pac::can1::Can1;
use crate::scu; 

#[cfg(target_arch = "tricore")]
use defmt::println; 

impl CanNode for Can0 {
    fn node_id(&self) -> u8 {
        0
    }
    #[inline]
    fn is_enabled(&self) -> bool {
        !unsafe { tc37x_pac::CAN0.clc().read() }.diss().get()
    }

    fn is_suspended(&self) -> bool {
        unsafe { tc37x_pac::CAN0.ocs().read() }.sussta().get()
    }
    fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();
        
        #[cfg(feature = "log")]
        println!("enable module watchdog passw: {:x}", passw);

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { tc37x_pac::CAN0.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }


    fn disable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { tc37x_pac::CAN0.clc().modify(|r| r.disr().set(true)) };

        while self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

    fn reset_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { tc37x_pac::CAN0.krst0().modify(|r| r.rst().set(true)) };
        unsafe { tc37x_pac::CAN0.krst1().modify(|r| r.rst().set(true)) };
        scu::wdt::set_cpu_endinit_inline(passw);

        while !unsafe { tc37x_pac::CAN0.krst0().read() }.rststat().get() {}

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { tc37x_pac::CAN0.krstclr().init(|r| r.clr().set(true)) };
        scu::wdt::set_cpu_endinit_inline(passw);
    }

    // fn set_clock_source(&self, clock_select: ClockSelect, clock_source: ClockSource) {
    //     let mut mcr = unsafe { tc37x_pac::CAN0.mcr().read() };
    //     mcr = mcr.ccce().set(true).ci().set(true);

    //     unsafe { tc37x_pac::CAN0.mcr().write(mcr) };

    //     mcr = mcr.clksel(clock_select.into()).set(clock_source.into());
    //     unsafe { tc37x_pac::CAN0.mcr().write(mcr) };

    //     mcr = mcr.ccce().set(false).ci().set(false);
    //     unsafe { tc37x_pac::CAN0.mcr().write(mcr) };
    // }

   
}

// TODO (annabo)
// replace CAN0 with CAN1 on implementation above. TBD with macros
// impl CanNode for Can1 {
//     fn nodeId(&self) -> u8 {
//         0
//     }
//     #[inline]
//     fn is_enabled(&self) -> bool {
//         !unsafe { tc37x_pac::CAN1.clc().read() }.diss().get()
//     }
//     fn is_suspended(&self) -> bool {
//         unsafe { tc37x_pac::CAN1.ocs().read() }.sussta().get()
//     }
// }

pub struct CanNodeHandler<T: CanNode> {
    node_id: u8,
    node: T,
}

impl CanNodeHandler<Can0> {
    pub const fn new() -> Self {
        Self {
            node_id: 0,
            node: tc37x_pac::CAN0,
        }
    }

    pub fn init_module(&self) {
        if !self.node.is_enabled() {
            #[cfg(feature = "log")]
            defmt::debug!("module was disabled, enabling it");
            self.node.enable_module();
        }
    }
}
