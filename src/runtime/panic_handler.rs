// link with defmt implementation
extern crate defmt_rtt;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    defmt::error!("Panic! {}", defmt::Display2Format(panic));
    #[allow(clippy::empty_loop)]
    loop {}
}
