#[cfg(feature = "panic_handler")]
#[cfg_attr(target_arch = "tricore", panic_handler)]
#[allow(unused)]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
