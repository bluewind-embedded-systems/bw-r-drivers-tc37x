//! Simple CAN example.

#![allow(unused_variables)]
#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::time::Duration;
use embedded_can::{ExtendedId, Frame};
use tc37x_hal::can::{CanModule, CanModuleId, CanNode, CanNodeConfig, NodeId};
use tc37x_hal::cpu::asm::enable_interrupts;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::log::info;
use tc37x_hal::util::wait_nop;
use tc37x_hal::{can, pac, ssw};

fn setup_can() -> Result<CanNode, ()> {
    let can_module = CanModule::new(CanModuleId::Can0);
    let mut can_module = can_module.enable()?;

    let can_node = can_module.take_node(NodeId::new(0))?;
    let mut can_node_config = CanNodeConfig::default();
    can_node_config.calculate_bit_timing_values = true;
    can_node_config.baud_rate.baud_rate = 1_000_000;
    can_node_config.baud_rate.sync_jump_with = 3;
    can_node_config.baud_rate.sample_point = 8_000;
    let can_node = can_node.configure(can_node_config)?;

    Ok(can_node)
}

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    #[cfg(feature = "log_with_env_logger")]
    env_logger::init();

    info!("Start example: can_send");

    info!("Enable interrupts");
    ssw::init_software();
    enable_interrupts();

    let gpio00 = pac::PORT_00.split();
    let mut led1 = gpio00.p00_5.into_push_pull_output();

    info!("Create can module ... ");

    let can_id: ExtendedId = ExtendedId::new(0x0CFE6E00).unwrap();
    let mut data: [u8; 8] = [0; 8];
    let test_frame = can::Frame::new(can_id, &data).unwrap();

    let can = match setup_can() {
        Ok(can) => can,
        Err(_) => loop {},
    };

    let mut count = 0;

    loop {
        if count < 255 {
            count += 1;
        } else {
            count = 0;
        }
        data[0] = count;

        info!("Sending message...");
        if can.transmit(&test_frame).is_err() {
            info!("Cannot send frame");
        }

        led1.set_high();
        wait_nop(Duration::from_millis(500));
        led1.set_low();
        wait_nop(Duration::from_millis(500));
    }
}
