use tc37x_pac::tracing::Reporter;

#[derive(Copy, Clone)]
pub struct DummyEffectReporter;

impl Reporter for DummyEffectReporter {
    fn read_volatile(&self, ptr: usize, len: usize) -> u32 {
        0
    }

    fn write_volatile(&self, ptr: usize, len: usize, val: u32) {}

    fn load_modify_store(&self, ptr: usize, val: u64) {}
}
