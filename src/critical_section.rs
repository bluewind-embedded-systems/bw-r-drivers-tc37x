use core::{arch::asm, panic::PanicInfo};
use critical_section::RawRestoreState;
use tc37x_rt::{
    asm_calls::read_cpu_core_id,
    isr::load_interrupt_table,
    wdtcon::{disable_cpu_watchdog, disable_safety_watchdog},
    *,
};

struct Section;
critical_section::set_impl!(Section);

unsafe impl critical_section::Impl for Section {
    unsafe fn acquire() -> RawRestoreState {
        unsafe { asm!("disable") };
        true
    }

    unsafe fn release(token: RawRestoreState) {
        if token {
            unsafe { asm!("enable") }
        }
    }
}
