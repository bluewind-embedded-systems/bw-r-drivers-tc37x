use std::any::Any;
use tc37x_pac::tracing::TraceGuard;

struct Reporter;

pub struct Report(TraceGuard);

impl Report {
    pub fn new() -> Self {
        let reporter = Reporter;
        let guard = TraceGuard::new(reporter);
        Self(guard)
    }
}

impl tc37x_pac::tracing::Reporter for Reporter {
    fn read_volatile(&self, ptr: usize, len: usize) -> u64 {
        println!("read_volatile 0x{:08X} len={}", ptr, len);
        0 // FIXME
    }

    fn write_volatile(&self, ptr: usize, len: usize, val: u64) {
        println!("write_volatile 0x{:08X} len={} val={}", ptr, len, val);
    }

    fn load_modify_store(&self, ptr: usize, val: u64) {
        println!("load_modify_store 0x{:08X} val={}", ptr, val);
    }
}
