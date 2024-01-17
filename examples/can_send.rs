//! Simple CAN example.

#![allow(unused_variables)]
#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::time::Duration;
use embedded_can::{ExtendedId, Frame};
use tc37x_hal::can::{
    CanModule, CanModuleId, CanNode, CanNodeConfig, DataFieldSize, NodeId, TxConfig, TxMode,
};
use tc37x_hal::cpu::asm::enable_interrupts;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::log::info;
use tc37x_hal::util::wait_nop_cycles;
use tc37x_hal::{can, pac, ssw};

fn setup_can() -> Result<CanNode, ()> {
    let can_module = CanModule::new(CanModuleId::Can0);
    let mut can_module = can_module.enable()?;

    let can_node = can_module.take_node(NodeId::new(0))?;
    let mut cfg = CanNodeConfig::default();
    cfg.calculate_bit_timing_values = true;
    cfg.baud_rate.baud_rate = 1_000_000;
    cfg.baud_rate.sync_jump_with = 3;
    cfg.baud_rate.sample_point = 8_000;
    cfg.tx = Some(TxConfig {
        mode: TxMode::DedicatedBuffers,
        dedicated_tx_buffers_number: 2,
        fifo_queue_size: 0,
        buffer_data_field_size: DataFieldSize::_8,
        event_fifo_size: 1,
    });
    let can_node = can_node.configure(cfg)?;

    Ok(can_node)
}

/// Initialize the STB pin for the CAN transceiver.
pub fn init_can_stb_pin() {
    let gpio20 = pac::PORT_20.split();
    let mut stb = gpio20.p20_6.into_push_pull_output();
    stb.set_low();
}

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    #[cfg(feature = "log_with_env_logger")]
    env_logger::init();

    info!("Start example: can_send");

    if let Err(_) = ssw::init_software() {
        info!("Error in ssw init");
        loop {}
    }

    enable_interrupts();

    let gpio00 = pac::PORT_00.split();
    let mut led1 = gpio00.p00_5.into_push_pull_output();

    info!("Create can module ... ");

    let can_id: ExtendedId = ExtendedId::new(0x0CFE6E00).unwrap();
    let mut data: [u8; 8] = [0; 8];
    let test_frame = can::Frame::new(can_id, &data).unwrap();

    init_can_stb_pin();

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

        led1.set_high();

        info!("Sending message...");
        if can.transmit(&test_frame).is_err() {
            info!("Cannot send frame");
        }

        wait_nop(Duration::from_millis(100));
        led1.set_low();
        wait_nop(Duration::from_millis(900));
    }
}

/// Wait for a number of cycles roughly calculated from a duration.
#[inline(always)]
pub fn wait_nop(period: Duration) {
    #[cfg(target_arch = "tricore")]
    {
        let ns = period.as_nanos() as u32;
        let n_cycles = ns / 920;
        wait_nop_cycles(n_cycles);
    }

    #[cfg(not(target_arch = "tricore"))]
    std::thread::sleep(period);
}
