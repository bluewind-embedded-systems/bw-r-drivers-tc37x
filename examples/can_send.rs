//! Simple CAN example.

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

#[cfg(target_arch = "tricore")]
use defmt::println;

use tc37x_hal::can::{ACanModule, CanModule0};
use tc37x_hal::cpu::asm::enable_interrupts;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::pac;
use tc37x_hal::ssw;
use tc37x_pac::RegisterValue;

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    println!("Start example: can_send");

    println!("Enable interrupts");
    enable_interrupts();

    let gpio00 = pac::PORT_00.split();
    let mut led1 = gpio00.p00_5.into_push_pull_output();

    println!("Create can module ... ");

    let can_module = CanModule0::default();
    let can_module_config = CanModuleConfig::default();
    let can_module = can_module.init(can_module_config)?;

    let can_node = can_module.get_node(0)?;
    let can_node_config = CanNodeConfig::default();
    let can_node = can_node.init(can_node_config)?;

    let mut data: [u8; 8] = [0; 8];

    let can = can_node;
    let mut count = 0;

    loop {
        if count < 255 {
            count += 1;
        } else {
            count = 0;
        }
        data[0] = count;

        let test_frame = Frame::new(0x0CFE6E00, data);
        can.transmit(&test_frame);
        led1.set_high();
        wait_nop(100000);
        led1.set_low();
        wait_nop(100000);
    }
}

fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        unsafe { core::arch::asm!("nop") };
    }
}
