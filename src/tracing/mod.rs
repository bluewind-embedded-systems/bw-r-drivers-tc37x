#![allow(unused)]

pub mod log;
pub mod print;

extern crate std;

use std::{
    sync::{Arc, Mutex},
    vec::Vec,
};

use crate::pac::tracing::Reporter;

#[derive(Debug, PartialEq, Eq)]
pub enum ReportEntry {
    Read(ReadEntry),
    Write(WriteEntry),
    LoadModifyStore(LoadModifyStoreEntry),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadEntry {
    addr: usize,
    len: usize,
    val: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WriteEntry {
    addr: usize,
    len: usize,
    val: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadModifyStoreEntry {
    addr: usize,
    val: u64,
}
