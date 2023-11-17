// global interrupts enable
#[inline]
pub fn enable_interrupts() {
    #[cfg(target_arch = "tricore")]
    unsafe {
        core::arch::asm!("enable");
    }
}

// global interrupts disable
#[inline]
pub fn disable_interrupts() {
    #[cfg(target_arch = "tricore")]
    unsafe {
        core::arch::asm!("disable");
    }
}

/** \brief FE1C, CPUx Core Identification Register */
#[allow(dead_code)]
const CPU_CORE_ID: u32 = 0xFE1C;

/**
 * Read CPU core id.
 */
#[inline]
#[cfg(target_arch = "tricore")]
pub fn read_cpu_core_id() -> u32 {
    #[allow(unused_assignments)]
    let value: u32;
    unsafe {
        core::arch::asm!("mfcr {0}, 0xFE1C", out(reg32) value);
    }
    value
}

#[inline]
#[cfg(not(target_arch = "tricore"))]
pub fn read_cpu_core_id() -> u32 {
    0
}
