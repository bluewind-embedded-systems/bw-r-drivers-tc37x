use tc37x_hal::gpio::GpioExt;
use tc37x_hal::tracing;
use tc37x_pac::PORT_00;

#[test]
fn test_pin_set_high() {
    let report = tracing::log::Report::new();

    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();
    output.set_low();

    insta::assert_display_snapshot!(report.get_log());
}
