//! Blinks an LED

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::arch::asm;
use tc37x_hal::pac;
use tc37x_pac::RegValue;
use tc37x_hal::gpio::GpioExt;

pub enum State {
    NotChanged = 0,
    High = 1,
    Low = 1 << 16,
    Toggled = (1 << 16) | 1,
}

fn port_00_set_state(index: usize, state: State) {
    let state = state as u32;
    unsafe {
        pac::PORT_00.omr().init(|mut r| {
            let data = r.data_mut_ref();
            *data = (state << index).into();
            r
        });
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

    unsafe {
        iocr.modify_atomic(|mut r| {
            *r.data_mut_ref() = (mode) << shift;
            *r.get_mask_mut_ref() = 0xFFu32 << shift;
            r
        })
    };
}

fn main() -> ! {
    defmt::info!("Hello world!");

    #[cfg(not(target_arch = "tricore"))]
    tc37x_hal::tracing::redirect_to_print();

    // TODO Adapt this example taken from https://github.com/stm32-rs/stm32f4xx-hal

    const PIN_INDEX: usize = 5;
    const OUTPUT_PUSH_PULL_GENERAL: u32 = 0x80;
    port_00_set_state(PIN_INDEX, State::High);
    port_00_set_mode(PIN_INDEX, OUTPUT_PUSH_PULL_GENERAL);

    // TODO Refactor to something similar to this:
    // TODO let p = pac::Peripherals::take().unwrap();
    // TODO let gpioc = p.GPIOC.split();
    // TODO let mut led = gpioc.pc13.into_push_pull_output();

    // let gpio00 = pac::PORT_00.split();
    // let mut led1 = gpio00.p00_5.into_push_pull_output();

    loop {
        defmt::info!("|");
        port_00_set_state(PIN_INDEX, State::High);
        // led1.set_high();
        wait_nop(100000);
        defmt::info!(".");
        port_00_set_state(PIN_INDEX, State::Low);
        // led1.set_low();
        wait_nop(100000);
    }
}

fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        unsafe { asm!("nop") };
    }
}
