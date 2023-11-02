use core::{arch::asm, panic::PanicInfo};
use critical_section::RawRestoreState;
use tc37x_rt::{
    asm_calls::read_cpu_core_id,
    isr::load_interrupt_table,
    wdtcon::{disable_cpu_watchdog, disable_safety_watchdog},
    *,
};

// link with defmt implementation
extern crate defmt_rtt;

// Configure the entry-point, pre and post init
pre_init!(pre_init_fn);
post_init!(post_init_fn);

#[allow(unused)]
fn pre_init_fn() {
    if read_cpu_core_id() == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

#[allow(unused)]
fn post_init_fn() {
    load_interrupt_table();
}

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

#[cfg_attr(target_arch = "tricore", panic_handler)]
#[allow(unused)]
fn panic(panic: &PanicInfo<'_>) -> ! {
    defmt::error!("Panic! {}", defmt::Display2Format(panic));

    #[allow(clippy::empty_loop)]
    loop {}
}

