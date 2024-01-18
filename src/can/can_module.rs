use super::can_node::{NewCanNode, Node, NodeId};
use crate::util::wait_nop_cycles;
use crate::{pac, scu};
use core::ops::Deref;
use tc37x_pac::can0::Mcr;

// TODO Remove Copy+Clone traits, we don't want to copy this
#[derive(Clone, Copy)]
struct CanRegisters(pac::can0::Can0);

impl CanRegisters {
    const fn can0() -> Self {
        Self(pac::CAN0)
    }
    const fn can1() -> Self {
        Self(unsafe { core::mem::transmute(pac::CAN1) })
    }
}

impl Deref for CanRegisters {
    type Target = pac::can0::Can0;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub enum ModuleId {
    Can0,
    Can1,
}

#[derive(Default)]
pub struct ModuleConfig {}

pub struct NewCanModule {
    id: ModuleId,
    inner: CanRegisters,
}

pub struct Module {
    id: ModuleId,
    inner: CanRegisters,
}

impl NewCanModule {
    pub fn enable(self) -> Result<Module, ()> {
        if !self.is_enabled() {
            self.enable_module();
        }

        Ok(Module {
            inner: self.inner,
            id: self.id,
        })
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

impl Module {
    pub const fn new(id: ModuleId) -> NewCanModule {
        let inner = match id {
            ModuleId::Can0 => CanRegisters::can0(),
            ModuleId::Can1 => CanRegisters::can1(),
        };

        // TODO Use id to select the correct CAN module
        NewCanModule {
            inner,
            id,
        }
    }

    pub fn take_node(&mut self, node_id: NodeId) -> Result<NewCanNode, ()> {
        // Instead of dealing with lifetimes, we just create a new instance of CanModule
        // TODO This is not ideal, but it works for now
        // TODO Remember the node has been taken and return None on next call
        let module = Module {
            inner: self.inner,
            id: self.id,
        };
        Ok(Node::new(module, node_id))
    }

    pub fn id(&self) -> ModuleId {
        self.id
    }

    pub(crate) fn set_clock_source(
        &self,
        clock_select: ClockSelect,
        clock_source: ClockSource,
    ) -> Result<(), ()> {
        let mcr = self.read_mcr();

        // Enable CCCE and CI
        let mcr = mcr.ccce().set(true).ci().set(true);
        self.write_mcr(mcr);

        // Select clock
        let mcr = match clock_select.0 {
            0 => mcr.clksel0().set(clock_source.into()),
            1 => mcr.clksel1().set(clock_source.into()),
            2 => mcr.clksel2().set(clock_source.into()),
            3 => mcr.clksel3().set(clock_source.into()),
            _ => unreachable!(),
        };

        self.write_mcr(mcr);

        // Disable CCCE and CI
        let mcr = mcr.ccce().set(false).ci().set(false);
        self.write_mcr(mcr);

        // TODO Is this enough or we need to wait until actual_clock_source == clock_source
        // Wait for clock switch
        wait_nop_cycles(10);

        // Check if clock switch was successful
        let mcr = self.read_mcr();

        let actual_clock_source = match clock_select.0 {
            0 => mcr.clksel0().get(),
            1 => mcr.clksel1().get(),
            2 => mcr.clksel2().get(),
            3 => mcr.clksel3().get(),
            _ => unreachable!(),
        };

        if actual_clock_source != clock_source.into() {
            return Err(());
        }

        Ok(())
    }

    pub(crate) fn registers(&self) -> &pac::can0::Can0 {
        &self.inner
    }

    fn read_mcr(&self) -> Mcr {
        unsafe { self.inner.mcr().read() }
    }

    fn write_mcr(&self, mcr: Mcr) {
        unsafe { self.inner.mcr().write(mcr) }
    }
}

pub(crate) struct ClockSelect(u8);

impl From<NodeId> for ClockSelect {
    fn from(node_id: NodeId) -> Self {
        Self(node_id.0)
    }
}

#[derive(Default, Clone, Copy)]
pub enum ClockSource {
    //TODO remove NoClock
    //NoClock,
    Asynchronous,
    Synchronous,
    #[default]
    Both,
}

impl From<ClockSource> for u8 {
    fn from(x: ClockSource) -> Self {
        match x {
            //ClockSource::NoClock => 0,
            ClockSource::Asynchronous => 1,
            ClockSource::Synchronous => 2,
            ClockSource::Both => 3,
        }
    }
}
