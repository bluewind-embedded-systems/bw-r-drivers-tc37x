pub(crate) fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        #[cfg(target_arch = "tricore")]
        unsafe {
            core::arch::asm!("nop")
        };
    }
}

// This is needed because tricore toolchain does not provide f32::abs method
pub(crate) trait F32Abs {
    fn abs(self) -> Self;
}

impl F32Abs for f32 {
    fn abs(self) -> Self {
        if self < 0.0 {
            -self
        } else {
            self
        }
    }
}
