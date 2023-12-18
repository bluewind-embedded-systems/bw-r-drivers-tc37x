use std::time::Duration;

pub fn wait_nop(period: Duration) {
    #[cfg(target_arch = "tricore")]
    {
        let n = period.as_micros() / 5;
        for _ in 0..n {
            unsafe { core::arch::asm!("nop") };
        }
    }

    #[cfg(not(target_arch = "tricore"))]
    std::thread::sleep(period);
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
