use std::any::Any;
use tc37x_pac::tracing::TraceGuard;

struct Reporter;

pub struct Report(TraceGuard);

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl Report {
    pub fn new() -> Self {
        let reporter = Reporter;
        let guard = TraceGuard::new(reporter);
        Self(guard)
    }
}

impl tc37x_pac::tracing::Reporter for Reporter {
    fn read_volatile(&self, addr: usize, len: usize) -> u64 {
        println!("r    0x{:08X} {:02}", addr, len);
        0
    }

    fn write_volatile(&self, addr: usize, len: usize, val: u64) {
        println!("w    0x{:08X} {:02} 0x{:08X}", addr, len, val);
    }

    fn load_modify_store(&self, addr: usize, val: u64) {
        let mask = (val >> 32);
        let val = (val & 0xFFFFFFFF);
        println!("ldms 0x{:08X} 0x{:08X} 0x{:08X}", addr, mask, val);
    }
}
