//! Blinks LED1 and LED2 on Aurix Lite Kit V2. Blinks faster when BUTTON1 is pressed.

#![cfg_attr(target_arch = "tricore", no_main)]
#![cfg_attr(target_arch = "tricore", no_std)]

#[cfg(target_arch = "tricore")]
bw_r_rt_example::entry!(main);

use bw_r_driver_tc37x::cpu::asm::read_cpu_core_id;
use bw_r_driver_tc37x::gpio::GpioExt;
use bw_r_driver_tc37x::log::info;
use bw_r_driver_tc37x::scu::wdt::{disable_cpu_watchdog, disable_safety_watchdog};
use bw_r_driver_tc37x::{pac, ssw};
use bw_r_rt_example::{isr::load_interrupt_table, post_init, pre_init};
use core::time::Duration;
use embedded_hal::digital::StatefulOutputPin;

pub enum State {
    NotChanged = 0,
    High = 1,
    Low = 1 << 16,
    Toggled = (1 << 16) | 1,
}

fn main() -> ! {
    #[cfg(not(target_arch = "tricore"))]
    let _report = bw_r_driver_tc37x::tracing::print::Report::new();

    #[cfg(feature = "log_with_env_logger")]
    env_logger::init();

    let gpio00 = pac::P00.split();

    let mut led1 = gpio00.p00_5.into_push_pull_output();
    let mut led2 = gpio00.p00_6.into_push_pull_output();
    let button1 = gpio00.p00_7.into_input();

    let mut was_pressed = false;

    loop {
        let is_pressed = button1.is_low();

        if is_pressed != was_pressed {
            was_pressed = is_pressed;
            if is_pressed {
                info!("Button pressed");
            } else {
                info!("Button released");
            }
        }

        let period = Duration::from_millis(if is_pressed { 50 } else { 500 });

        // Test set_low
        led1.set_low();

        // Test toggle
        led2.toggle();

        info!("Wait for {:?}", period);
        wait_nop(period);
        info!("Wait done");

        // Test high
        led1.set_high();

        // Test is_set_high
        if led1.is_set_high().unwrap_or_default() {
            led2.set_low();
        }

        // Test is_set_low
        if led1.is_set_low().unwrap_or_default() {
            led2.set_high();
        }

        wait_nop(period);
    }
}

/// Wait for a number of cycles roughly calculated from a duration.
// TODO Are we sure we want to publish this function?
#[inline(always)]
fn wait_nop(period: Duration) {
    #[cfg(target_arch = "tricore")]
    {
        use bw_r_driver_tc37x::util::wait_nop_cycles;
        let ns = period.as_nanos() as u32;
        let n_cycles = ns / 1412;
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

#[cfg(target_arch = "tricore")]
#[allow(unused_variables)]
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    #[cfg(feature = "log_with_defmt")]
    defmt::error!("Panic! {}", defmt::Display2Format(panic));
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(feature = "log_with_defmt")]
mod critical_section_impl {
    use core::arch::asm;
    use critical_section::RawRestoreState;

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
}
