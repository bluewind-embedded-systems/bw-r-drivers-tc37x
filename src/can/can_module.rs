use super::can_node::{NewCanNode, Node, NodeId};
use crate::util::wait_nop_cycles;
use crate::{pac, scu};
use core::marker::PhantomData;
use core::ops::Deref;

#[derive(Clone, Copy)]
pub enum ModuleId {
    Can0,
    Can1,
}

#[derive(Default)]
pub struct ModuleConfig {}

pub struct NewCanModule<T>(PhantomData<T>);

pub struct Module<T>(PhantomData<T>);

macro_rules! can_module {
    ($reg:ident, $m:ident, $Reg:ty, $id: expr) => {
impl NewCanModule<$Reg> {
    pub fn enable(self) -> Result<Module<$Reg>, ()> {
        if !self.is_enabled() {
            self.enable_module();
        }

        let module = Module::<$Reg>(PhantomData);

        Ok(module)
    }

    pub fn is_enabled(&self) -> bool {
        !unsafe { $reg.clc().read() }.diss().get()
    }

    pub fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { $reg.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }
}

impl Module<$Reg> {
    pub const fn new() -> NewCanModule<$Reg> {
        NewCanModule::<$Reg>(PhantomData)
    }

    pub fn take_node(&mut self, node_id: NodeId) -> Result<NewCanNode, ()> {
        // Instead of dealing with lifetimes, we just create a new instance of CanModule
        // TODO This is not ideal, but it works for now
        // TODO Remember the node has been taken and return None on next call
        // TODO Avoid transmute, return the right type
        let module = Module::<$Reg>(PhantomData);
        let module = unsafe { core::mem::transmute(module) };
        Ok(Node::new(module, node_id))
    }

    pub fn id(&self) -> ModuleId {
        $id
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

    // TODO Return the right type (avoid transmute)
    pub(crate) fn registers(&self) -> &pac::can0::Can0 {
        unsafe { core::mem::transmute(&$reg) }
    }

    fn read_mcr(&self) -> $m::Mcr {
        unsafe { $reg.mcr().read() }
    }

    fn write_mcr(&self, mcr: $m::Mcr) {
        unsafe { $reg.mcr().write(mcr) }
    }
}
    };
}

use crate::pac::can0;
use crate::pac::can1;
use crate::pac::can0::Can0;
use crate::pac::can1::Can1;
use crate::pac::CAN0;
use crate::pac::CAN1;
can_module!(CAN0, can0, Can0, ModuleId::Can0);
can_module!(CAN1, can1, Can1, ModuleId::Can1);

// TODO Should remember if the module has been taken
pub fn can_module0() -> NewCanModule<Can0> {
    NewCanModule::<Can0>(PhantomData)
}

// TODO Should remember if the module has been taken
pub fn can_module1() -> NewCanModule<Can1> {
    NewCanModule::<Can1>(PhantomData)
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
