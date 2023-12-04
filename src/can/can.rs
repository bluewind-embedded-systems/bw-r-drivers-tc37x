// CanNode trait impl to be moved on a separate file (annabo)
pub trait ACanModule {
    fn node_id(&self) -> u8;
    fn is_enabled(&self) -> bool;
    fn is_suspended(&self) -> bool;

    fn enable_module(&self);
    fn disable_module(&self);
    fn reset_module(&self);
    fn init_module(&self);
    //fn set_clock_source(&self, clock_select: ClockSelect, clock_source: ClockSource);
}

pub struct CanModule0 {
    inner: tc37x_pac::can0::Can0,
}

// TODO (annabo) use tc37x_pac::can1::Can1; impl CanModule for Can1
use crate::scu;

#[cfg(feature = "log")]
#[cfg(target_arch = "tricore")]
use defmt::println;
use embedded_can::nb::Can;

impl ACanModule for CanModule0 {
    fn node_id(&self) -> u8 {
        0
    }

    #[inline]
    fn is_enabled(&self) -> bool {
        !unsafe { self.inner.clc().read() }.diss().get()
    }

    fn is_suspended(&self) -> bool {
        unsafe { self.inner.ocs().read() }.sussta().get()
    }
   
    fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        #[cfg(feature = "log")]
        println!("enable module watchdog passw: {:x}", passw);

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { self.inner.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

    fn disable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { self.inner.clc().modify(|r| r.disr().set(true)) };

        while self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

    fn reset_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { self.inner.krst0().modify(|r| r.rst().set(true)) };
        unsafe { self.inner.krst1().modify(|r| r.rst().set(true)) };
        scu::wdt::set_cpu_endinit_inline(passw);

        while !unsafe { self.inner.krst0().read() }.rststat().get() {}

        scu::wdt::clear_cpu_endinit_inline(passw);
        unsafe { self.inner.krstclr().init(|r| r.clr().set(true)) };
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
    fn init_module(&self) {
        if !self.is_enabled() {
            #[cfg(feature = "log")]
            defmt::debug!("module was disabled, enabling it");
            self.enable_module();
        }
    }
}

impl CanModule0{

    pub fn new() -> Self {
        let m = Self {
            inner: tc37x_pac::CAN0,
        }; 
        m.enable_module(); 
        m
        

    }

}