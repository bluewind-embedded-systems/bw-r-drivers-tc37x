#![allow(unused)]

#[attr(panic_handler)]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
