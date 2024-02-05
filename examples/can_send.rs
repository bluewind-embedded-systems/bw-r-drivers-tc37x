//! Simple CAN example.

#![allow(unused_variables)]
#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);

use core::time::Duration;
use embedded_can::ExtendedId;
use tc37x_driver::can::pin_map::*;
use tc37x_driver::can::*;
use tc37x_driver::cpu::asm::enable_interrupts;
use tc37x_driver::cpu::asm::read_cpu_core_id;
use tc37x_driver::gpio::GpioExt;
use tc37x_driver::log::info;
use tc37x_driver::scu::wdt::{disable_cpu_watchdog, disable_safety_watchdog};
use tc37x_driver::{pac, ssw};
use tc37x_pac::can0::{Can0, N as Can0Node};
use tc37x_pac::can1::{Can1, N as Can1Node};
use tc37x_rt::{isr::load_interrupt_table, post_init, pre_init};

pub static CAN0_NODE0_NEW_MSG: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn __INTERRUPT_HANDLER_2() {
    CAN0_NODE0_NEW_MSG.store(true, Ordering::SeqCst);
}

fn setup_can0() -> Option<Node<Can0Node, Can0>> {
    let can_module = Module::new(Module0);
    let mut can_module = can_module.enable();

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

    cfg.rx = Some(RxConfig {
        mode: RxMode::SharedFifo0,
        buffer_data_field_size: DataFieldSize::_8,
        fifo0_data_field_size: DataFieldSize::_8,
        fifo1_data_field_size: DataFieldSize::_8,
        fifo0_operating_mode: RxFifoMode::Blocking,
        fifo1_operating_mode: RxFifoMode::Blocking,
        fifo0_watermark_level: 0,
        fifo1_watermark_level: 0,
        fifo0_size: 4,
        fifo1_size: 0,
    });

    cfg.pins = Some(Pins {
        tx: PIN_TX_0_0_P20_8,
        rx: PIN_RX_0_0_P20_7,
    });

    can_module.take_node(Node0, cfg)
}

fn setup_can1() -> Option<Node<Can1Node, Can1>> {
    let can_module = Module::new(Module1);
    let mut can_module = can_module.enable();

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

    cfg.rx = Some(RxConfig {
        mode: RxMode::SharedFifo0,
        buffer_data_field_size: DataFieldSize::_8,
        fifo0_data_field_size: DataFieldSize::_8,
        fifo1_data_field_size: DataFieldSize::_8,
        fifo0_operating_mode: RxFifoMode::Blocking,
        fifo1_operating_mode: RxFifoMode::Blocking,
        fifo0_watermark_level: 0,
        fifo1_watermark_level: 0,
        fifo0_size: 4,
        fifo1_size: 0,
    });

    cfg.pins = Some(Pins {
        tx: PIN_TX_1_0_P00_0,
        rx: PIN_RX_1_0_P13_1,
    });

    can_module.take_node(Node0, cfg)
}

/// Initialize the STB pin for the CAN transceiver.
fn init_can_stb_pin() {
    let gpio20 = pac::PORT_20.split();
    let mut stb = gpio20.p20_6.into_push_pull_output();
    stb.set_low();
}

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_driver::tracing::print::Report::new();

    #[cfg(feature = "log_with_env_logger")]
    env_logger::init();

    info!("Start example: can_send");

    info!("Enable interrupts");
    enable_interrupts();

    info!("Setup notification LED");
    let gpio00 = pac::PORT_00.split();
    let mut led1 = gpio00.p00_5.into_push_pull_output();

    info!("Initialize CAN transceiver");
    init_can_stb_pin();

    info!("Create CAN module ... ");
    let can0 = match setup_can0() {
        Some(can) => can,
        None => {
            info!("Can initialization error");
            loop {}
        }
    };

    let can1 = match setup_can1() {
        Some(can) => can,
        None => {
            info!("Can initialization error");
            loop {}
        }
    };

    info!("Define a message to send");
    let tx_msg_id: MessageId = {
        let id = 0x0CFE6E00;
        let id: ExtendedId = ExtendedId::new(id).unwrap().into();
        id.into()
    };

    info!("Allocate a buffer for the message data");
    let mut tx_msg_data: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    loop {
        // Transmit a different message each time (changing the first byte)
        tx_msg_data[0] = tx_msg_data[0].wrapping_add(1);

        let tx_frame = Frame::new(tx_msg_id, tx_msg_data.as_slice()).unwrap();

        if can0.transmit(&tx_frame).is_err() {
            info!("Cannot send frame");
        }

        // if can1.transmit(&tx_frame).is_err() {
        //     info!("Cannot send frame");
        // }

        led1.set_high();
        wait_nop(Duration::from_millis(100));
        led1.set_low();
        wait_nop(Duration::from_millis(900));

        let can0_node0_received = CAN0_NODE0_NEW_MSG.load(Ordering::SeqCst);
        if can0_node0_received {
            info!("msg received");
            CAN0_NODE0_NEW_MSG.store(false, Ordering::SeqCst);
            can0.clear_interrupt_flag(Interrupt::RxFifo0newMessage);
        }
    }
}

/// Wait for a number of cycles roughly calculated from a duration.
#[inline(always)]
pub fn wait_nop(period: Duration) {
    #[cfg(target_arch = "tricore")]
    {
        use tc37x_driver::util::wait_nop_cycles;
        let ns = period.as_nanos() as u32;
        let n_cycles = ns / 920;
        wait_nop_cycles(n_cycles);
    }

    #[cfg(not(target_arch = "tricore"))]
    std::thread::sleep(period);
}

// Note: without this, the watchdog will reset the CPU
pre_init!(pre_init_fn);
fn pre_init_fn() {
    if read_cpu_core_id() == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

post_init!(post_init_fn);
fn post_init_fn() {
    if let Err(_) = ssw::init_clock() {
        info!("Error in ssw init");
        loop {}
    }

    load_interrupt_table();
}

#[allow(unused_variables)]
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    defmt::error!("Panic! {}", defmt::Display2Format(panic));
    #[allow(clippy::empty_loop)]
    loop {}
}

use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::RawRestoreState;
use tc37x_driver::cpu::Priority;

struct Section;

critical_section::set_impl!(Section);

unsafe impl critical_section::Impl for Section {
    unsafe fn acquire() -> RawRestoreState {
        unsafe { asm!("disable") };
        true
    }

    unsafe fn release(token: RawRestoreState) {
        if token {
            unsafe { asm!("enable") }
        }
    }
}
