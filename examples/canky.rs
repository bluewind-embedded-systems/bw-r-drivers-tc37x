//! Blinks an LED

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
tc37x_rt::entry!(main);
#[cfg(target_arch = "tricore")]
use defmt::println;

use core::arch::asm;
use tc37x_hal::cpu::asm;
use tc37x_hal::gpio::GpioExt;
use tc37x_hal::pac;
use tc37x_pac::RegisterValue;
use tc37x_hal::ssw::init_software; 
use tc37x_hal::can::{CanModule0, ACanModule};

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
    #[cfg(not(target_arch = "tricore"))]
    let _report = tc37x_hal::tracing::print::Report::new();

    println!("Start example: CanKy");
    println!("Enable interrupts");
    //ssw::init_software();
    tc37x_hal::cpu::asm::enable_interrupts();

    let gpio00 = pac::PORT_00.split();

    let mut led1 = gpio00.p00_5.into_push_pull_output();
    let mut led2 = gpio00.p00_6.into_push_pull_output();
    let mut i = 0;
    
    println!("Create can module ... ");
    let can_module = CanModule0::new(); 
    println!("Can module init .. ");
    can_module.init_module(); 

    if can_module.is_enabled()
    {
        println!("Can module is now enabled! ")
    }
    else 
    {
        println!("Can module NOT enabled! Something went wrong!")
    }
    loop {
        led1.set_low();
        led2.set_high();
        wait_nop(100000);


        println!("running {}", i);
        i += 1;

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

/* (TODO) to be moved in a separate module! (annabo) */


pub struct CanCommunication; 

impl CanCommunication{
    fn new(&self) -> Self{
        Self{}
    }
}