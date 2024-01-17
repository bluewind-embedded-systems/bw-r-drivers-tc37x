use core::time::Duration;

#[allow(unused)]
#[inline(always)]
pub fn wait_nop_cycles(n_cycles: u32) {
    #[cfg(target_arch = "tricore")]
    for _ in 0..n_cycles {
        unsafe { core::arch::asm!("nop") };
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
