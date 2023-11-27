/* From ILLD file: 
*  file IfxCpu.h
*  brief CPU  basic functionality
*  ingroup IfxLld_Cpu
*  version iLLD_1_0_1_12_0
*  copyright Copyright (c) 2019 Infineon Technologies AG. All rights reserved.
*/

// global interrupts enable
#[inline]
pub fn enable_interrupts() {
    #[cfg(target_arch = "tricore")]
    unsafe {
        core::arch::asm!("enable");
    }
}

// TODO (annabo) // to be tested
/* ILLD FUNCTION 
 * IFX_INLINE boolean IfxCpu_disableInterrupts(void)
 * {
 *     boolean enabled;
 *     enabled = IfxCpu_areInterruptsEnabled();
 *     __disable();
 *     __nop();
 *     return enabled;
 * }
 * */

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
 * IFX_INLINE IfxCpu_Id IfxCpu_getCoreId(void)
 * {
 *     Ifx_CPU_CORE_ID reg;
 *     reg.U = __mfcr(CPU_CORE_ID);
 *     return (IfxCpu_Id)reg.B.CORE_ID;
 * }
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
