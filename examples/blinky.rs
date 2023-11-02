//! Blinks an LED

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use pac::{RegValue, PORT_00};
use tc37x_hal::pac;

fn port_00_set_high(index: usize) {
    unsafe {
        pac::PORT_00.omr().init(|mut r| {
            let data = r.data_mut_ref();
            *data = (1u32 << index).into();
            r
        });
    };
}

fn port_00_set_low(index: usize) {
    unsafe {
        pac::PORT_00.omr().init(|mut r| {
            let data = r.data_mut_ref();
            *data = (0u32 << index).into();
            r
        });
    };
}

fn port_00_set_mode(index: usize, mode: u32) {
    let ioc_index = index / 4;
    let shift = (index & 0x3) * 8;

    let iocr: pac::Reg<pac::port_00::Iocr0, pac::RW> = unsafe {
        let iocr0 = PORT_00.iocr0();
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
    port_00_set_mode(PIN_INDEX, OUTPUT_PUSH_PULL_GENERAL);

    // TODO Refactor to something similar to this:
    // TODO let p = pac::Peripherals::take().unwrap();
    // TODO let gpioc = p.GPIOC.split();
    // TODO let mut led = gpioc.pc13.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            // TODO led.set_high();
            port_00_set_high(PIN_INDEX);
        }
        for _ in 0..10_000 {
            // TODO led.set_low();
            port_00_set_low(PIN_INDEX);
        }
    }
}
