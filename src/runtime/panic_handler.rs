// link with defmt implementation
extern crate defmt_rtt;

use core::{arch::asm, panic::PanicInfo};
use critical_section::RawRestoreState;
use tc37x_rt::{
    asm_calls::read_cpu_core_id,
    isr::load_interrupt_table,
    wdtcon::{disable_cpu_watchdog, disable_safety_watchdog},
    *,
};

#[panic_handler]
#[allow(unused)]
fn panic(panic: &PanicInfo<'_>) -> ! {
    defmt::error!("Panic! {}", defmt::Display2Format(panic));

    #[allow(clippy::empty_loop)]
    loop {}
}
