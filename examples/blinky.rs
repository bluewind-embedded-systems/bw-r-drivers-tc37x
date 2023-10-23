#![no_std]
#![no_main]
#[cfg_attr(target_arch = "tricore", panic_handler)]
#[allow(unused)]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {

    #[allow(clippy::empty_loop)]
    loop {}
}

tc37x_rt::entry!(main);

fn main() -> ! {
    loop{}
}