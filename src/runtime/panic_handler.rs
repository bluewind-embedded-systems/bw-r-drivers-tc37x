use crate::log::error;
use core::panic::PanicInfo;

#[allow(unused_variables)]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    error!("Panic! {}", defmt::Display2Format(panic));
    #[allow(clippy::empty_loop)]
    loop {}
}
