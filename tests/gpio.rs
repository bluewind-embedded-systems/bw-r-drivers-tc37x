use embedded_hal::digital::PinState;
use tc37x::P01;
use tc37x::{P00, P20};

use bw_r_driver_tc37x::gpio::{ErasedPin, GpioExt};
use bw_r_driver_tc37x::tracing;
use tracing::log::Report;

#[test]
fn test_pin_set_high_and_low() {
    let report = Report::new();

    let port = P00.split();
    let mut output = port.p00_5.into_push_pull_output();

    output.set_high();
    output.set_low();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_pin_set_two_pins_same_port_high() {
    let report = Report::new();

    let port = P00.split();
    let mut p00_5 = port.p00_5.into_push_pull_output();
    let mut p00_6 = port.p00_6.into_push_pull_output();

    p00_5.set_high();
    p00_6.set_high();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_pin_set_two_pins_on_two_ports_high() {
    let report = Report::new();

    let port = P00.split();
    let gpio01 = P01.split();
    let mut p00_5 = port.p00_5.into_push_pull_output();
    let mut p01_7 = gpio01.p01_7.into_push_pull_output();

    p00_5.set_high();
    p01_7.set_high();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn avoid_side_effects_when_mode_does_not_change() {
    let report = Report::new();

    let port = P00.split();
    let pin = port.p00_5.into_push_pull_output();
    let _pin = pin.into_push_pull_output();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_input_pin() {
    let report = Report::new();

    let port = P00.split();
    let pin = port.p00_7.into_input();

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000000000000);
    let is_high = pin.is_high();
    assert!(!is_high);

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000010000000);
    let is_high = pin.is_high();
    assert!(is_high);

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_input_pin_pull_up() {
    let _report = Report::new();

    let port = P00.split();
    let _pin = port.p00_7.into_pull_up_input();

    // TODO Review report. Not sure about PCx
    // insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_output_pin_type_erasure_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let mut output = output.erase_number();

    output.set_high();
    output.set_low();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_output_pin_type_erasure_port_and_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let mut output = output.erase();

    output.set_high();
    output.set_low();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_input_pin_type_erasure_number() {
    let report = Report::new();

    let port = P00.split();
    let pin = port.p00_7.into_input();
    let pin = pin.erase_number();

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000000000000);
    let is_high = pin.is_high();
    assert!(!is_high);

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000010000000);
    let is_high = pin.is_high();
    assert!(is_high);

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn test_input_pin_type_erasure_port_and_number() {
    let report = Report::new();

    let port = P00.split();
    let pin = port.p00_7.into_input();
    let pin = pin.erase();

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000000000000);
    let is_high = pin.is_high();
    assert!(!is_high);

    report.expect_read(0xF003A024, 4, 0b00000000000000000000000010000000);
    let is_high = pin.is_high();
    assert!(is_high);

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn toggle_output_pin() {
    let report = Report::new();

    let port = P00.split();
    let mut output = port.p00_5.into_push_pull_output();

    output.toggle();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn toggle_output_pin_type_erasure_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let mut output = output.erase_number();

    output.toggle();

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn toggle_output_pin_type_erasure_port_and_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let mut output = output.erase();

    output.toggle();

    insta::assert_snapshot!(report.take_log());
}

// Set the pin high by using StatefulOutputPin interface
// This is needed to ensure the StatefulOutputPin trait is implemented
fn stateful_output_pin_is_set_high(pin: impl embedded_hal::digital::StatefulOutputPin) -> bool {
    pin.is_set_high().unwrap()
}

#[test]
fn toggle_stateful_output_pin_stateful() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();

    report.expect_read(0xF003A000, 4, 0b00000000000000000000000000100000);

    assert!(stateful_output_pin_is_set_high(output));

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn toggle_stateful_output_pin_type_erasure_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let output = output.erase_number();

    report.expect_read(0xF003A000, 4, 0b00000000000000000000000000100000);

    assert!(stateful_output_pin_is_set_high(output));

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn toggle_stateful_output_pin_type_erasure_port_and_number() {
    let report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let output = output.erase();

    report.expect_read(0xF003A000, 4, 0b00000000000000000000000000100000);

    assert!(stateful_output_pin_is_set_high(output));

    insta::assert_snapshot!(report.take_log());
}

#[test]
fn type_erasure_with_into() {
    let _report = Report::new();

    let port = P00.split();
    let output = port.p00_5.into_push_pull_output();
    let output = output.erase();
    let _output: ErasedPin<_> = output;
}

#[test]
fn pin_can_type_match_with_peripheral() {
    let _report = Report::new();

    use self::mock_can::*;

    let port = P20.split();
    let rx = port.p20_7;
    let tx = port.p20_8;

    let _can = Can::<tc37x::can0::Can0>::new(tx, rx);
}

mod mock_can {
    use bw_r_driver_tc37x::gpio::*;

    pub struct Can<CAN: alt::CanCommon> {
        _tx_pin: CAN::Tx,
        _rx_pin: CAN::Rx,
    }

    impl<CAN: alt::CanCommon> Can<CAN> {
        pub fn new(tx_pin: impl Into<CAN::Tx>, rx_pin: impl Into<CAN::Rx>) -> Self {
            Can {
                _tx_pin: tx_pin.into(),
                _rx_pin: rx_pin.into(),
            }
        }
    }
}

// TODO Discussed during meeting 2023-11-24
// User case is: I want to control many pins in the same port at once, e.g. to
// implement bit banging.
// #[test]
// fn test_gpio_syncronous_update_all_pins_same_mode() {
//     let report = Report::new();
//
//     let port = P00.split();
//     let pins = (port.p00_1, port.p00_6, port.p00_7);
//     let group = PinGroup::new(pins);
//     let group = group.into_push_pull_output();
//     group.set_high();
//     group.set_low();
//     group.set_raw(0b1100010);
//     group.set_values((true, true, true));
// }
//
// #[test]
// fn test_gpio_syncronous_update_mixed_pins_mode() {
//     let report = Report::new();
//
//     let port = P00.split();
//
//     let pins = (
//         port.p00_1.into_input(),
//         port.p00_6.into_push_pull_output(),
//         port.p00_7.into_push_pull_output(),
//     );
//
//     let group = PinGroup::new(pins);
//
//     // TODO Check if setting an input pin is not UB
//     // Only 6 and 7 should be set
//     group.set_high();
//     group.set_low();
//     group.set_raw(0b1100010);
//     group.set_values((true, true, true));
// }

// User case is: I want to control many pins in the same port at once, e.g. to
// implement bit banging.
#[test]
fn test_gpio_outport_array() {
    use bw_r_driver_tc37x::gpio::group::PinArray;

    let report = Report::new();
    let port = P00.split();

    let mut group = PinArray([
        port.p00_1.into_push_pull_output().erase_number(),
        port.p00_6.into_push_pull_output().erase_number(),
        port.p00_7.into_push_pull_output().erase_number(),
    ]);

    group.set_high();
    group.set_state([PinState::High, PinState::High, PinState::High]);
    group.set_low();
    group.set_state([PinState::Low, PinState::Low, PinState::Low]);

    insta::assert_snapshot!(report.take_log());
}

// User case is: I want to control many pins in the same port at once, e.g. to
// implement bit banging.
#[test]
fn test_gpio_outport_tuple() {
    use bw_r_driver_tc37x::gpio::group::PinGroup;

    let report = Report::new();

    report.comment("Configure pins");

    let port = P00.split();

    let mut group = (
        port.p00_1.into_push_pull_output(),
        port.p00_6.into_push_pull_output(),
        port.p00_7.into_push_pull_output(),
    )
        .into_pin_group();

    report.comment("Set all pins high");
    group.set_high();

    report.comment("Set all pins low");
    group.set_low();

    insta::assert_snapshot!(report.take_log());

    // Make sure set_high is equivalent to set_state and write with all pins high
    {
        group.set_high();
        let set_high_log = report.take_log();

        group.set_state([PinState::High, PinState::High, PinState::High]);
        let set_state_log = report.take_log();

        group.write(0xFFFFFFFF);
        let write_log = report.take_log();

        assert_eq!(set_high_log, set_state_log);
        assert_eq!(set_high_log, write_log);
    }

    // Make sure set_low is equivalent to set_state and write with all pins low
    {
        group.set_low();
        let set_low_log = report.take_log();

        group.set_state([PinState::Low, PinState::Low, PinState::Low]);
        let set_state_log = report.take_log();

        group.write(0x00000000);
        let write_log = report.take_log();

        assert_eq!(set_low_log, set_state_log);
        assert_eq!(set_low_log, write_log);
    }
}
