use tc37x_pac::tracing::Reporter;

#[derive(Copy, Clone)]
pub struct PrintEffectReporter;

impl Reporter for PrintEffectReporter {
    fn read_volatile(&self, ptr: usize, len: usize) -> u32 {
        println!("read_volatile 0x{:08X} len={}", ptr, len);
        0 // FIXME
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
    pac::tracing::set_effect_reporter(Box::new(PrintEffectReporter));
}
