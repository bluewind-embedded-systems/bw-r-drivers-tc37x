#![allow(unused)]

use core::cell::RefCell;
use dummy::DummyEffectReporter;
use tc37x as pac;

pub mod dummy;
pub mod log;
pub mod print;

extern crate std;

use std::{
    sync::{Arc, Mutex},
    vec::Vec,
};

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

pub trait Reporter: Sync + Send {
    fn read_volatile(&self, ptr: usize, len: usize) -> u64;
    fn write_volatile(&self, ptr: usize, len: usize, val: u64);
    fn load_modify_store(&self, ptr: usize, val: u64);
}

thread_local! {
static EFFECT_REPORTER: RefCell<Box<dyn Reporter>> = RefCell::new(Box::new(DummyEffectReporter));
}

fn report<T>(f: impl FnOnce(&dyn Reporter) -> T) -> T {
    EFFECT_REPORTER.with(|r| f(r.borrow().as_ref()))
}

pub struct TraceGuard;

impl TraceGuard {
    pub fn new<T: Reporter + 'static>(reporter: T) -> Self {
        eprintln!("TraceGuard::new");

        pac::tracing::set_read_fn(read_volatile);
        pac::tracing::set_write_fn(write_volatile);
        pac::tracing::set_ldmst_fn(load_modify_store);

        EFFECT_REPORTER.with(|r| *r.borrow_mut() = Box::new(reporter));
        Self
    }
}

impl Drop for TraceGuard {
    fn drop(&mut self) {
        EFFECT_REPORTER.with(|r| *r.borrow_mut() = Box::new(DummyEffectReporter));
    }
}

pub(crate) fn load_modify_store(addr: usize, val: u64) {
    EFFECT_REPORTER.with(|r| r.borrow().load_modify_store(addr, val));
}

pub(crate) fn write_volatile(addr: usize, len: usize, val: u64) {
    EFFECT_REPORTER.with(|r| r.borrow().write_volatile(addr, len, val));
}

pub(crate) fn read_volatile(addr: usize, len: usize) -> u64 {
    EFFECT_REPORTER.with(|r| r.borrow().read_volatile(addr, len))
}
