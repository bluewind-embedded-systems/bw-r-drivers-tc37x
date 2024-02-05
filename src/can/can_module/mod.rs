mod service_request;

use super::can_node::{Node, NodeConfig};

use crate::can::Priority;
use crate::can::Tos;
use crate::can::{InterruptLine, NodeId};
use crate::util::wait_nop_cycles;
use crate::{pac, scu};
use core::marker::PhantomData;
use pac::hidden::CastFrom;

pub trait ModuleId {}

pub struct Module0;
impl ModuleId for Module0 {}

pub struct Module1;
impl ModuleId for Module1 {}

// Type states for Module
pub struct Disabled;
pub struct Enabled;

pub struct Module<ModuleId, Reg, State> {
    nodes_taken: [bool; 4],
    _phantom: PhantomData<(ModuleId, Reg, State)>,
}

impl<ModuleId, Reg> Module<ModuleId, Reg, Disabled> {
    /// Create a new (disabled) CAN module
    pub fn new(_module_id: ModuleId) -> Self {
        Self {
            nodes_taken: [false; 4],
            _phantom: PhantomData,
        }
    }
}

macro_rules! impl_can_module {
    ($module_reg:path, $($m:ident)::+, $ModuleReg:ty, $ModuleId: ty) => {
        impl Module<$ModuleId, $ModuleReg, Disabled> {
            fn is_enabled(&self) -> bool {
                !unsafe { $module_reg.clc().read() }.diss().get()
            }

            /// Enable the CAN module
            pub fn enable(self) -> Module<$ModuleId, $ModuleReg, Enabled> {
                scu::wdt::clear_cpu_endinit_inline();

                unsafe { $module_reg.clc().modify_atomic(|r| r.disr().set(false)) };
                while !self.is_enabled() {}

                scu::wdt::set_cpu_endinit_inline();

                Module::<$ModuleId, $ModuleReg, Enabled> {
                    nodes_taken: [false; 4],
                    _phantom: PhantomData,
                }
            }
        }

        impl Module<$ModuleId, $ModuleReg, Enabled> {
            /// Take ownership of a CAN node and configure it
            pub fn take_node<I>(&mut self, node_id: I, cfg: NodeConfig<$ModuleReg, I>) -> Option<Node<$($m)::+::N, $ModuleReg>> where I: NodeId {
                let node_index = node_id.as_index();

                // Check if node is already taken, return None if it is
                if self.nodes_taken[node_index] {
                    return None;
                }

                // Mark node as taken
                self.nodes_taken[node_index] = true;

                // Create node
                Node::<$($m)::+::N, $ModuleReg>::new(self, node_id, cfg).ok()
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

            pub(crate) fn set_interrupt(
                &self,
                line: InterruptLine,
                priority: Priority,
                tos: Tos,
            ) {
                <$ModuleId>::service_request(line).enable(priority, tos)
            }

            pub(crate) fn registers(&self) -> &$ModuleReg {
                &$module_reg
            }

            fn read_mcr(&self) -> $($m)::+::Mcr {
                unsafe { $module_reg.mcr().read() }
            }

            fn write_mcr(&self, mcr: $($m)::+::Mcr) {
                unsafe { $module_reg.mcr().write(mcr) }
            }

            pub(crate) fn ram_base_address(&self) -> usize {
                $module_reg.0 as usize
            }
        }
    };
}

impl_can_module!(pac::CAN0, pac::can0, pac::can0::Can0, Module0);
impl_can_module!(pac::CAN1, pac::can1, pac::can1::Can1, Module1);

pub(crate) struct ClockSelect(pub(crate) u8);

impl<T> From<T> for ClockSelect
where
    T: NodeId,
{
    fn from(value: T) -> Self {
        ClockSelect(value.as_index() as u8)
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
