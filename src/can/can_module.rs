use super::can_node::{CanNode, NodeId};
use crate::can::NewCanNode;
use crate::{pac, scu};

#[derive(Default)]
pub struct CanModuleConfig {}

pub struct NewCanModule {
    inner: pac::can0::Can0,
}

pub struct CanModule {
    inner: pac::can0::Can0,
}

impl NewCanModule {
    pub fn configure(self, _config: CanModuleConfig) -> Result<CanModule, ()> {
        if !self.is_enabled() {
            self.enable_module();
        }

        Ok(CanModule { inner: self.inner })
    }

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
}

impl CanModule {
    pub const fn new(_index: usize) -> NewCanModule {
        // TODO Use index
        NewCanModule { inner: pac::CAN0 }
    }

    pub fn take_node(&mut self, node_id: NodeId) -> Result<NewCanNode, ()> {
        // Instead of dealing with lifetimes, we just create a new instance of CanModule
        // TODO This is not ideal, but it works for now
        // TODO Remember the node has been taken and return None on next call
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

    pub(crate) fn registers(&self) -> &pac::can0::Can0 {
        &self.inner
    }
}

pub(crate) struct ClockSelect(u8);

impl From<NodeId> for ClockSelect {
    fn from(node_id: NodeId) -> Self {
        Self(node_id.0)
    }
}

#[derive(Default)]
pub enum ClockSource {
    #[default]
    NoClock,
    Asynchronous,
    Synchronous,
    Both,
}

impl From<ClockSource> for u8 {
    fn from(x: ClockSource) -> Self {
        match x {
            ClockSource::NoClock => 0,
            ClockSource::Asynchronous => 1,
            ClockSource::Synchronous => 2,
            ClockSource::Both => 3,
        }
    }
}
