use super::can_node::{Node, NodeConfig, NodeId};

use crate::util::wait_nop_cycles;
use crate::{pac, scu};
use core::marker::PhantomData;

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

pub struct Module<Reg, State = Disabled> {
    nodes_taken: [bool; 4],
    _phantom: PhantomData<(Reg, State)>,
}

macro_rules! impl_can_module {
    ($reg:path, $($m:ident)::+, $Reg:ty, $id: expr) => {
        impl Module<$Reg, Disabled> {
            pub const fn new() -> Self {
                Self {
                    nodes_taken: [false; 4],
                    _phantom: PhantomData,
                }
            }

            fn is_enabled(&self) -> bool {
                !unsafe { $reg.clc().read() }.diss().get()
            }

            pub fn enable(self) -> Module<$Reg, Enabled> {
                let passw = scu::wdt::get_cpu_watchdog_password();
                scu::wdt::clear_cpu_endinit_inline();

                unsafe { $reg.clc().modify_atomic(|r| r.disr().set(false)) };
                while !self.is_enabled() {}

                scu::wdt::set_cpu_endinit_inline();

                Module::<$Reg, Enabled> {
                    nodes_taken: [false; 4],
                    _phantom: PhantomData,
                }
            }
        }

        impl Module<$Reg, Enabled> {
            // This method is private to prevent the user from creating a new instance of an enabled module
            const fn new() -> Self {
                Self {
                    nodes_taken: [false; 4],
                    _phantom: PhantomData,
                }
            }

            pub fn take_node(&mut self, node_id: NodeId, cfg: NodeConfig) -> Option<Node<$($m)::+::N, $Reg>> {
                let node_index = match node_id {
                    NodeId::Node0 => 0,
                    NodeId::Node1 => 1,
                    NodeId::Node2 => 2,
                    NodeId::Node3 => 3,
                };

                if self.nodes_taken[node_index] {
                    return None;
                }

                self.nodes_taken[node_index] = true;

                Node::<$($m)::+::N, $Reg>::new(self, node_id, cfg).ok()
            }

            pub fn id(&self) -> ModuleId {
                $id
            }

            pub(crate) fn set_clock_source(
                &self,
                clock_select: ClockSelect,
                clock_source: ClockSource,
            ) -> Result<(), ()> {
                use $($m)::+::mcr::{Ccce, Ci, Clksel0, Clksel1, Clksel2, Clksel3};

                let mcr = self.read_mcr();

                // Enable CCCE and CI
                let mcr = mcr
                    .ccce()
                    .set($($m)::+::mcr::Ccce::CONST_11)
                    .ci()
                    .set($($m)::+::mcr::Ci::CONST_11);
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

            fn read_mcr(&self) -> $($m)::+::Mcr {
                unsafe { $reg.mcr().read() }
            }

            fn write_mcr(&self, mcr: $($m)::+::Mcr) {
                unsafe { $reg.mcr().write(mcr) }
            }

            pub(crate) fn ram_base_address(&self) -> usize {
                $reg.0 as usize
            }
        }
    };
}

impl_can_module!(pac::CAN0, pac::can0, pac::can0::Can0, ModuleId::Can0);
impl_can_module!(pac::CAN1, pac::can1, pac::can1::Can1, ModuleId::Can1);

pub(crate) struct ClockSelect(u8);

impl From<NodeId> for ClockSelect {
    fn from(node_id: NodeId) -> Self {
        Self(node_id.into())
    }
}

#[derive(Default, Clone, Copy)]
pub enum ClockSource {
    Asynchronous,
    Synchronous,
    #[default]
    Both,
}

impl From<ClockSource> for u8 {
    fn from(x: ClockSource) -> Self {
        match x {
            ClockSource::Asynchronous => 1,
            ClockSource::Synchronous => 2,
            ClockSource::Both => 3,
        }
    }
}
