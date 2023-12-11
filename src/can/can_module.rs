use super::can_node::{CanNode, NodeId};
use crate::{pac, scu};

#[derive(Default)]
pub struct CanModuleConfig {}

pub struct CanModule {
    inner: pac::can0::Can0,
}

impl CanModule {
    pub const fn new(_index: usize) -> Self {
        // TODO Use index
        Self { inner: pac::CAN0 }
    }

    pub fn init(self, _config: CanModuleConfig) -> Result<CanModule, ()> {
        if !self.is_enabled() {
            self.enable_module();
        }

        Ok(self)
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        !unsafe { self.inner.clc().read() }.diss().get()
    }

    pub fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { self.inner.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

    pub fn get_node(&mut self, node_id: NodeId) -> Result<CanNode, ()> {
        // Instead of dealing with lifetimes, we just create a new instance of CanModule
        // TODO This is not ideal, but it works for now
        let module = CanModule { inner: self.inner };
        Ok(CanNode::new(module, node_id))
    }

    pub(crate) fn set_clock_source(&self, clock_select: ClockSelect, clock_source: ClockSource) {
        let mcr = unsafe { self.inner.mcr().read() };

        // Enable CCCE and CI
        let mcr = mcr.ccce().set(true).ci().set(true);
        unsafe { self.inner.mcr().write(mcr) };

        // Select clock
        let mcr = match clock_select.0 {
            0 => mcr.clksel0().set(clock_source.into()),
            1 => mcr.clksel1().set(clock_source.into()),
            2 => mcr.clksel2().set(clock_source.into()),
            3 => mcr.clksel3().set(clock_source.into()),
            _ => unreachable!(),
        };

        unsafe { tc37x_pac::CAN0.mcr().write(mcr) };

        // Disable CCCE and CI
        let mcr = mcr.ccce().set(false).ci().set(false);
        unsafe { self.inner.mcr().write(mcr) };
    }
}

pub(crate) struct ClockSelect(u8);

impl From<NodeId> for ClockSelect {
    fn from(node_id: NodeId) -> Self {
        Self(node_id.0)
    }
}

pub enum ClockSource {
    NoClock,
    Asynchronous,
    Synchronous,
    Both,
}
