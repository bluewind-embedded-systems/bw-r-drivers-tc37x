//! Blinks LED1 and LED2 on Aurix Lite Kit V2. Blinks faster when BUTTON1 is pressed.

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::arch::asm;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::pac;

pub enum State {
    NotChanged = 0,
    High = 1,
    Low = 1 << 16,
    Toggled = (1 << 16) | 1,
}

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    let gpio00 = pac::PORT_00.split();

    let mut led1 = gpio00.p00_5.into_push_pull_output();
    let mut led2 = gpio00.p00_6.into_push_pull_output();
    let button1 = gpio00.p00_7.into_input();

    loop {
        let period = if button1.is_high() { 100_000 } else { 25_000 };
        led1.set_low();
        led2.toggle();
        wait_nop(period);
        led1.set_high();
        led2.toggle();
        wait_nop(period);
    }
}

fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        unsafe { asm!("nop") };
    }
}
