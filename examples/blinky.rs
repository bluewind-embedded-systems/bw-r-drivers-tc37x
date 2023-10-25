//! Blinks an LED

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
    // TODO Adapt this example taken from https://github.com/stm32-rs/stm32f4xx-hal

    // let p = pac::Peripherals::take().unwrap();
    //
    // let gpioc = p.GPIOC.split();
    // let mut led = gpioc.pc13.into_push_pull_output();
    //
    loop {
        //     for _ in 0..10_000 {
        //         led.set_high();
        //     }
        //     for _ in 0..10_000 {
        //         led.set_low();
        //     }
    }
}
