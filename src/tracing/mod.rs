#![allow(unused)]

pub mod log;
pub mod print;

extern crate std;

use std::{
    sync::{Arc, Mutex},
    vec::Vec,
};

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
