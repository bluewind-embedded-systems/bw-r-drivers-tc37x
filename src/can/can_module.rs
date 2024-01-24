use super::can_node::{NewCanNode, Node, NodeId};
use crate::log::info;
use crate::util::wait_nop_cycles;
use crate::{pac, scu};
use core::marker::PhantomData;
use core::ops::Deref;
use pac::hidden::CastFrom;

#[derive(Clone, Copy)]
pub enum ModuleId {
    Can0,
    Can1,
}

#[derive(Default)]
pub struct ModuleConfig {}

// Type states for Module
pub struct Disabled;
pub struct Enabled;

pub struct Module<Reg, State = Disabled>(PhantomData<(Reg, State)>);

macro_rules! can_module {
    ($reg:ident, $m:ident, $Reg:ty, $id: expr) => {
        impl Module<$Reg, Disabled> {
            pub const fn new() -> Module<$Reg, Disabled> {
                Module::<$Reg, Disabled>(PhantomData)
            }
        }

        impl Module<$Reg, Disabled> {
            fn is_enabled(&self) -> bool {
                !unsafe { $reg.clc().read() }.diss().get()
            }

            pub fn enable(self) -> Module<$Reg, Enabled> {
                let passw = scu::wdt::get_cpu_watchdog_password();
                scu::wdt::clear_cpu_endinit_inline(passw);

                unsafe { $reg.clc().modify_atomic(|r| r.disr().set(false)) };
                while !self.is_enabled() {}

                scu::wdt::set_cpu_endinit_inline(passw);

                Module::<$Reg, Enabled>(PhantomData)
            }
        }

        impl Module<$Reg, Enabled> {
            pub fn take_node(&mut self, node_id: NodeId) -> Result<NewCanNode<$m::N, $Reg>, ()> {
                // Instead of dealing with lifetimes, we just create a new instance of CanModule
                // TODO This is not ideal, but it works for now
                // TODO Remember the node has been taken and return None on next call
                let module = Module::<$Reg, Enabled>(PhantomData);
                Ok(Node::<$m::N, $Reg>::new(module, node_id))
            }

            pub fn id(&self) -> ModuleId {
                $id
            }

            pub(crate) fn set_clock_source(
                &self,
                clock_select: ClockSelect,
                clock_source: ClockSource,
            ) -> Result<(), ()> {
                use $m::mcr::{Ccce, Ci, Clksel0, Clksel1, Clksel2, Clksel3};

                let mcr = self.read_mcr();

                // Enable CCCE and CI
                let mcr = mcr
                    .ccce()
                    .set($m::mcr::Ccce::CONST_11)
                    .ci()
                    .set($m::mcr::Ci::CONST_11);
                self.write_mcr(mcr);

                // Select clock
                let clock_source: u8 = clock_source.into();

                let mcr = match clock_select.0 {
                    0 => mcr.clksel0().set(Clksel0::cast_from(clock_source.into())),
                    1 => mcr.clksel1().set(Clksel1::cast_from(clock_source.into())),
                    2 => mcr.clksel2().set(Clksel2::cast_from(clock_source.into())),
                    3 => mcr.clksel3().set(Clksel3::cast_from(clock_source.into())),
                    _ => unreachable!(),
                };

                self.write_mcr(mcr);

                // Disable CCCE and CI
                let mcr = mcr.ccce().set(Ccce::CONST_00).ci().set(Ci::CONST_00);
                self.write_mcr(mcr);

                // TODO Is this enough or we need to wait until actual_clock_source == clock_source
                // Wait for clock switch
                wait_nop_cycles(10);

                // Check if clock switch was successful
                let mcr = self.read_mcr();

                let actual_clock_source = match clock_select.0 {
                    0 => mcr.clksel0().get().0,
                    1 => mcr.clksel1().get().0,
                    2 => mcr.clksel2().get().0,
                    3 => mcr.clksel3().get().0,
                    _ => unreachable!(),
                };

                if actual_clock_source != clock_source {
                    return Err(());
                }

                Ok(())
            }

            pub(crate) fn registers(&self) -> &$Reg {
                &$reg
            }

            fn read_mcr(&self) -> $m::Mcr {
                unsafe { $reg.mcr().read() }
            }

            fn write_mcr(&self, mcr: $m::Mcr) {
                unsafe { $reg.mcr().write(mcr) }
            }

            pub(crate) fn ram_base_address(&self) -> usize {
                $reg.0 as usize
            }
        }
    };
}

use crate::pac::can0;
use crate::pac::can0::Can0;
use crate::pac::can1;
use crate::pac::can1::Can1;
use crate::pac::CAN0;
use crate::pac::CAN1;

can_module!(CAN0, can0, Can0, ModuleId::Can0);
can_module!(CAN1, can1, Can1, ModuleId::Can1);

pub(crate) struct ClockSelect(u8);

impl From<NodeId> for ClockSelect {
    fn from(node_id: NodeId) -> Self {
        Self(node_id.into())
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
