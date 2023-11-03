#![allow(unused)]

mod dummy;
pub mod log;
pub mod print;

extern crate std;

use std::{
    sync::{Arc, Mutex},
    vec::Vec,
};

use crate::pac::tracing::Reporter;
use crate::tracing::dummy::DummyEffectReporter;

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

pub fn test_with(reporter: impl FnOnce() -> Box<dyn Reporter>, test_body: impl FnOnce() -> ()) {
    let reporter = reporter();
    crate::pac::tracing::set_effect_reporter(reporter);
    test_body();
    crate::pac::tracing::set_effect_reporter(Box::new(DummyEffectReporter));
}
