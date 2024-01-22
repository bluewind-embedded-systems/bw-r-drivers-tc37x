//! Simple CAN example.

#![allow(unused_variables)]
#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::time::Duration;
use embedded_can::ExtendedId;
use tc37x_hal::can::can_module0;
use tc37x_hal::can::{
    AutoBitTiming, BitTimingConfig, DataFieldSize, Frame, Node, NodeConfig, NodeId, TxConfig,
    TxMode,
};
use tc37x_hal::cpu::asm::enable_interrupts;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::log::info;
use tc37x_hal::{pac, ssw};
use tc37x_pac::can0;

fn setup_can() -> Result<Node<can0::N, can0::Can0>, ()> {
    let can_module = can_module0();
    let mut can_module = can_module.enable()?;

    let can_node_id = NodeId::Node0;
    let can_node = can_module.take_node(can_node_id)?;
    let mut cfg = NodeConfig::default();

    cfg.baud_rate = BitTimingConfig::Auto(AutoBitTiming {
        baud_rate: 1_000_000,
        sample_point: 8_000,
        sync_jump_width: 3,
    });

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
fn init_can_stb_pin() {
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

    // info!("CAN0 MCR: {:?}", unsafe { CAN0.mcr().read() }.get_raw());
    // info!("CAN1 MCR: {:?}", unsafe { CAN0.mcr().read() }.bits());

    // loop {
    //
    // }

    init_can_stb_pin();

    let can = match setup_can() {
        Ok(can) => can,
        Err(_) => loop {},
    };

    let tx_msg_id: ExtendedId = ExtendedId::new(0x0CFE6E00).unwrap().into();
    let tx_msg_id = tx_msg_id.into();
    let mut tx_msg_data: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    loop {
        led1.set_high();

        // Transmit a different message each time (changing the first byte)
        tx_msg_data[0] = tx_msg_data[0].wrapping_add(1);

        let tx_frame = Frame::new(tx_msg_id, tx_msg_data.as_slice()).unwrap();

        if can.transmit(&tx_frame).is_err() {
            info!("Cannot send frame");
        }

        wait_nop(Duration::from_millis(10));
        led1.set_low();
        wait_nop(Duration::from_millis(90));
    }
}

/// Wait for a number of cycles roughly calculated from a duration.
#[inline(always)]
pub fn wait_nop(period: Duration) {
    #[cfg(target_arch = "tricore")]
    {
        use tc37x_hal::util::wait_nop_cycles;
        let ns = period.as_nanos() as u32;
        let n_cycles = ns / 920;
        wait_nop_cycles(n_cycles);
    }

    #[cfg(not(target_arch = "tricore"))]
    std::thread::sleep(period);
}
