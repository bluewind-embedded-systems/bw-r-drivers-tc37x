//! Blinks an LED

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::arch::asm;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::pac;
use tc37x_pac::RegisterValue;

pub enum State {
    NotChanged = 0,
    High = 1,
    Low = 1 << 16,
    Toggled = (1 << 16) | 1,
}

fn port_00_set_state(index: usize, state: State) {
    let state = state as u32;
    unsafe {
        pac::PORT_00
            .omr()
            .init(|r| r.set_raw((state << index).into()));
    };
}

fn port_00_set_mode(index: usize, mode: u32) {
    let ioc_index = index / 4;
    let shift = (index & 0x3) * 8;

    let iocr: pac::Reg<pac::port_00::Iocr0, pac::RW> = unsafe {
        let iocr0 = pac::PORT_00.iocr0();
        let addr: *mut u32 = core::mem::transmute(iocr0);
        let addr = addr.add(ioc_index as _);
        core::mem::transmute(addr)
    };

    use tc37x_pac::common::hidden::RegValue;

    unsafe {
        iocr.modify_atomic(|mut r| {
            *r.data_mut_ref() = (mode) << shift;
            *r.get_mask_mut_ref() = 0xFFu32 << shift;
            r
        })
    };
}

fn main() -> ! {
    // defmt::info!("Hello world!");

    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    // TODO Adapt this example taken from https://github.com/stm32-rs/stm32f4xx-hal

    const LED1_PIN_INDEX: usize = 5;
    const LED2_PIN_INDEX: usize = 6;

    const OUTPUT_PUSH_PULL_GENERAL: u32 = 0x80;

    // TODO Remove this
    port_00_set_state(LED1_PIN_INDEX, State::High);
    port_00_set_mode(LED1_PIN_INDEX, OUTPUT_PUSH_PULL_GENERAL);

    // TODO Remove this
    port_00_set_state(LED2_PIN_INDEX, State::High);
    port_00_set_mode(LED2_PIN_INDEX, OUTPUT_PUSH_PULL_GENERAL);

    // TODO Refactor to something similar to this:
    let gpio00 = pac::PORT_00.split();
    let mut led1 = gpio00.p00_5.into_push_pull_output();
    let mut led2 = gpio00.p00_6.into_push_pull_output();

    loop {
        // defmt::info!("|");
        led1.set_low();
        led2.set_high();
        wait_nop(100000);
        // defmt::info!(".");
        led1.set_high();
        led2.set_low();
        wait_nop(100000);
    }
}

fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        unsafe { asm!("nop") };
    }
}
