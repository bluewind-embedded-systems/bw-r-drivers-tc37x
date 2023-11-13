use tc37x_pac::PORT_00;
use tc37x_pac::PORT_01;

use tc37x_hal::gpio::GpioExt;
use tc37x_hal::tracing;

#[test]
fn test_pin_set_high_and_low() {
    let report = tracing::log::Report::new();

    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();
    output.set_low();

    insta::assert_display_snapshot!(report.get_log());
}

#[test]
fn test_pin_set_two_pins_same_port_high() {
    let report = tracing::log::Report::new();

    let gpio00 = PORT_00.split();
    let mut p00_5 = gpio00.p00_5.into_push_pull_output();
    let mut p00_6 = gpio00.p00_6.into_push_pull_output();

    p00_5.set_high();
    p00_6.set_high();

    insta::assert_display_snapshot!(report.get_log());
}

#[test]
fn test_pin_set_two_pins_on_two_ports_high() {
    let report = tracing::log::Report::new();

    let gpio00 = PORT_00.split();
    let gpio01 = PORT_01.split();
    let mut p00_5 = gpio00.p00_5.into_push_pull_output();
    let mut p01_7 = gpio01.p01_7.into_push_pull_output();

    p00_5.set_high();
    p01_7.set_high();

    insta::assert_display_snapshot!(report.get_log());
}

#[test]
fn avoid_side_effects_when_mode_does_not_change() {
    let report = tracing::log::Report::new();

    let port = PORT_00.split();
    let pin = port.p00_5.into_push_pull_output();
    let _pin = pin.into_push_pull_output();

    insta::assert_display_snapshot!(report.get_log());
}
