// CanNode trait impl to be moved on a separate file (annabo)
pub trait ACanModule {
    fn node_id(&self) -> u8;
    fn is_enabled(&self) -> bool;
    fn is_suspended(&self) -> bool;

    fn enable_module(&self);
    fn disable_module(&self);
    fn reset_module(&self);
    fn init_module(&self);
    fn set_clock_source(&self, clock_select: CanClockSelect, clock_source: CanClockSource);
    fn get_module_frequency(&self) -> f32;
}

pub struct CanModule0 {
    inner: tc37x_pac::can0::Can0,
}

#[repr(u8)]
pub enum CanClockSource {
    NoClock = 0,      /* \brief No clock is switched on */
    Asynchronous = 1, /* \brief The Asynchronous clock source is switched on */
    Synchronous = 2,  /* \brief The Synchronous clock source is switched on */
    Both = 3,         /* \brief Both clock sources are switched on */
}
impl From<CanClockSource> for u8 {
    fn from(value: CanClockSource) -> Self {
        value as _
    }
}
pub enum CanClockSelect {
    _0 = 0, /* \brief clock selection 0  */
    _1 = 1, /* \brief clock selection 1  */
    _2 = 2, /* \brief clock selection 2  */
    _3 = 3, /* \brief clock selection 3  */
}

// TODO (annabo) use tc37x_pac::can1::Can1; impl CanModule for Can1
use crate::scu;

#[cfg(feature = "log")]
#[cfg(target_arch = "tricore")]
use defmt::println;

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

    fn set_clock_source(&self, clock_select: CanClockSelect, clock_source: CanClockSource) {
        let mut mcr = unsafe { self.inner.mcr().read() };
        mcr = mcr.ccce().set(true).ci().set(true);

        unsafe { self.inner.mcr().write(mcr) };

        match clock_select {
            CanClockSelect::_0 => mcr.clksel0().set(clock_source.into()),
            CanClockSelect::_1 => mcr.clksel1().set(clock_source.into()),
            CanClockSelect::_2 => mcr.clksel2().set(clock_source.into()),
            CanClockSelect::_3 => mcr.clksel3().set(clock_source.into()),
        };

        unsafe { tc37x_pac::CAN0.mcr().write(mcr) };

        mcr = mcr.ccce().set(false).ci().set(false);
        unsafe { self.inner.mcr().write(mcr) };
    }

    fn init_module(&self) {
        if !self.is_enabled() {
            #[cfg(feature = "log")]
            defmt::debug!("module was disabled, enabling it");
            self.enable_module();
        }
    }

    fn get_module_frequency(&self) -> f32 {
        let value = unsafe { tc37x_pac::SCU.ccucon1().read().clkselmcan().get() };
        match value {
            0 => todo!(),
            _ => todo!(),
        };
    }
}

impl CanModule0 {
    pub fn new() -> Self {
        let m = Self {
            inner: tc37x_pac::CAN0,
        };
        m.enable_module();
        m
    }
}
