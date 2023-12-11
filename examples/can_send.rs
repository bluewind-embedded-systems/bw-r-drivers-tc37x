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

    println!("Start example: CanKy");

    println!("Enable interrupts");
    enable_interrupts();

    let gpio00 = pac::PORT_00.split();

    let mut led1 = gpio00.p00_5.into_push_pull_output();

    println!("Create can module ... ");

    // cannot create a module if already taken
    let can_module = CanModule0::new(); // get an enabled module
    println!("Can module init .. ");
    can_module.init_module();

    //ottengo un nodo
    //let node: uninitializedNode = can_module.get_node(0);
    //let initedNode: InitedNode = node.init(config{configuration constructor})

    // cannot take a node if already taken
    // can_module.get_node(0);

    // let msg_id =  0;
    // let msg = Message::TxMessage::new(msg_id)
    // node.send_message(msg);

    if can_module.is_enabled() {
        println!("Can module is now enabled! ")
    } else {
        println!("Can module NOT enabled! Something went wrong!")
    }

    loop {
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

/* (TODO) to be moved in a separate module! (annabo) */

pub struct CanCommunication;

impl CanCommunication {
    fn new(&self) -> Self {
        Self {}
    }
}
