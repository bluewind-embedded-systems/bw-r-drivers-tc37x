use super::Reporter;

pub struct DummyEffectReporter;

impl Reporter for DummyEffectReporter {
    fn read_volatile(&self, _ptr: usize, _len: usize) -> u64 {
        0
    }

    fn write_volatile(&self, _ptr: usize, _len: usize, _val: u64) {}

    fn load_modify_store(&self, _ptr: usize, _val: u64) {}
}
