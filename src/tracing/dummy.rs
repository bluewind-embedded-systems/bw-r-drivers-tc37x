//! Dummy effect reporter for testing purposes. It does not record any effects.
//! and always returns 0 when reading from memory.
//! It is used as a default reporter when no other reporter is provided.

use super::Reporter;

pub struct DummyEffectReporter;

impl Reporter for DummyEffectReporter {
    fn read_volatile(&self, _ptr: usize, _len: usize) -> u64 {
        0
    }

    fn write_volatile(&self, _ptr: usize, _len: usize, _val: u64) {}

    fn load_modify_store(&self, _ptr: usize, _val: u64) {}
}
