// use tc37x_pac::{self as pac};
// use tc37x_pac::{can*}

// pub enum CanNumber {
//     _0,
//     _1,
// }

// #[derive(Clone, Copy)]
// pub struct Can {
//     inner: pac::can0::Can0,
// }

// impl Can {
//     pub const fn new(number: CanNumber) -> Self {
//         let inner: pac::can0::Can0 = unsafe {
//             match number {
//                 CanNumber::_0 => pac::CAN0,
//                 CanNumber::_1 => core::mem::transmute(pac::CAN1),
//             }
//         };

//         Self { inner }
//     }
// }

// // IfxLld_Can_Std_Module_Functions
// impl Can {
//     pub const fn get_ptr(&self) -> *mut u8 {
//         unsafe { core::mem::transmute(self.inner) }
//     }

//     #[inline]
//     pub fn is_enabled(&self) -> bool {
//         !unsafe { self.inner.clc().read() }.diss().get()
//     }

//     pub fn is_suspended(&self) -> bool {
//         unsafe { self.inner.ocs().read() }.sussta().get()
//     }

//     pub fn get_node(&self, node_id: NodeId) -> Node {
//         Node::new(node_id)
//     }

//     pub fn enable_module(&self) {
//         let passw = scu::wdt::get_cpu_watchdog_password();
//         #[cfg(feature = "log")]
//         defmt::debug!("enable module watchdog passw: {:x}", passw);

//         scu::wdt::clear_cpu_endinit_inline(passw);

//         unsafe { self.inner.clc().modify_atomic(|r| r.disr().set(false)) };
//         while !self.is_enabled() {}

//         scu::wdt::set_cpu_endinit_inline(passw);
//     }

//     pub fn disable_module(&self) {
//         let passw = scu::wdt::get_cpu_watchdog_password();

//         scu::wdt::clear_cpu_endinit_inline(passw);
//         unsafe { self.inner.clc().modify(|r| r.disr().set(true)) };

//         while self.is_enabled() {}

//         scu::wdt::set_cpu_endinit_inline(passw);
//     }

//     pub fn reset_module(&self) {
//         let passw = scu::wdt::get_cpu_watchdog_password();

//         scu::wdt::clear_cpu_endinit_inline(passw);
//         unsafe { self.inner.krst0().modify(|r| r.rst().set(true)) };
//         unsafe { self.inner.krst1().modify(|r| r.rst().set(true)) };
//         scu::wdt::set_cpu_endinit_inline(passw);

//         while !unsafe { self.inner.krst0().read() }.rststat().get() {}

//         scu::wdt::clear_cpu_endinit_inline(passw);
//         unsafe { self.inner.krstclr().init(|r| r.clr().set(true)) };
//         scu::wdt::set_cpu_endinit_inline(passw);
//     }

//     pub fn set_clock_source(&self, clock_select: ClockSelect, clock_source: ClockSource) {
//         let mut mcr = unsafe { self.inner.mcr().read() };
//         mcr = mcr.ccce().set(true).ci().set(true);

//         unsafe { self.inner.mcr().write(mcr) };

//         mcr = mcr.clksel(clock_select.into()).set(clock_source.into());
//         unsafe { self.inner.mcr().write(mcr) };

//         mcr = mcr.ccce().set(false).ci().set(false);
//         unsafe { self.inner.mcr().write(mcr) };
//     }
// }

// impl Can {
//     #[allow(unused)]
//     pub fn get_src(&self, line: InterruptLine) -> Src {
//         let can_idx = unsafe {
//             let can0: usize = core::mem::transmute(pac::CAN0);
//             let can1: usize = core::mem::transmute(pac::CAN1);
//             let current: usize = core::mem::transmute(self.inner);
//             match current {
//                 can0 => 0,
//                 can1 => 1,
//                 _ => unimplemented!(),
//             }
//         };
//         let can_src_off: usize = src::offsets::CAN * (1 + can_idx);
//         Src::new(can_src_off + line.0 as usize * 4)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn test_set_clock_source_reg() {
//         use core::mem::size_of_val;
//         use pac::RegValue;
//         extern crate std;
//         use crate::reporter::*;

//         let reporter = LogEffectReporter::default();
//         pac::tracing::set_effect_reporter(std::boxed::Box::new(reporter.clone()));

//         let module = Can::new(CanNumber::_0);
//         let clock_select = ClockSelect::_0;
//         let clock_source = ClockSource::Both;
//         module.set_clock_source(clock_select, clock_source);

//         let logs = reporter.get_logs();
//         let expected_data = pac::can0::Mcr::default()
//             .ccce()
//             .set(true)
//             .ci()
//             .set(true)
//             .clksel(clock_select.into())
//             .set(clock_source.into())
//             .data();

//         assert_eq!(logs.len(), 4);
//         assert_eq!(
//             logs[2],
//             ReportData::new(
//                 ReportAction::Write(expected_data),
//                 pac::CAN0.mcr().ptr() as usize,
//                 size_of_val(&expected_data)
//             )
//         );
//     }
// }
