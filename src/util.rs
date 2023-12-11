use core::arch::asm;

pub fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        #[cfg(target_arch = "tricore")]
        unsafe {
            asm!("nop")
        };
    }
}
