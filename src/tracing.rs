#![allow(unused)]

extern crate std;

use std::{
    sync::{Arc, Mutex},
    vec::Vec,
};

#[cfg(feature = "defmt")]
use defmt::println;

#[cfg(not(feature = "defmt"))]
use std::println;

use crate::pac::tracing::Reporter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportData {
    pub addr: usize,
    pub len: usize,
    pub action: ReportAction,
}

impl ReportData {
    pub fn new(action: ReportAction, addr: usize, len: usize) -> Self {
        Self { addr, len, action }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportAction {
    Read,
    Write(u32),
    LoadModifyStore(u64),
}

// TODO Remove, should be injectable
pub const READ_VALUE: u32 = 0x0;

#[derive(Default, Clone)]
pub struct LogEffectReporter(Arc<Mutex<Vec<ReportData>>>);

impl LogEffectReporter {
    pub fn get_logs(&self) -> Vec<ReportData> {
        let mut g = self.0.lock().unwrap();
        let len = g.len();
        g.drain(0..len).collect()
    }

    fn push(&self, report: ReportData) {
        self.0.lock().unwrap().push(report);
    }

    fn report(&self, action: ReportAction, addr: usize, len: usize) {
        self.push(ReportData::new(action, addr, len))
    }
}

impl Reporter for LogEffectReporter {
    fn read_volatile(&self, addr: usize, len: usize) -> u32 {
        self.report(ReportAction::Read, addr, len);
        READ_VALUE
    }

    fn write_volatile(&self, addr: usize, len: usize, val: u32) {
        self.report(ReportAction::Write(val), addr, len);
    }

    fn load_modify_store(&self, addr: usize, val: u64) {
        self.report(
            ReportAction::LoadModifyStore(val),
            addr,
            core::mem::size_of::<u64>(),
        );
    }
}

#[derive(Default, Copy, Clone)]
pub struct PrintEffectReporter;

impl Reporter for PrintEffectReporter {
    fn read_volatile(&self, ptr: usize, len: usize) -> u32 {
        println!("read_volatile 0x{:08X} len={}", ptr, len);
        READ_VALUE
    }

    fn write_volatile(&self, ptr: usize, len: usize, val: u32) {
        println!("write_volatile 0x{:08X} len={} val={}", ptr, len, val);
    }

    fn load_modify_store(&self, ptr: usize, val: u64) {
        println!("load_modify_store 0x{:08X} val={}", ptr, val);
    }
}

pub fn redirect_to_print() {
    use std::boxed::Box;
    use crate::pac;
    let reporter = PrintEffectReporter::default();
    pac::tracing::set_effect_reporter(Box::new(reporter.clone()));
}
