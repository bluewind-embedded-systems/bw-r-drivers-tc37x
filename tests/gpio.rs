use tc37x_hal::gpio::GpioExt;
use tc37x_hal::tracing;
use tc37x_hal::tracing::{ReportAction, ReportData};
use tc37x_pac::PORT_00;

#[test]
fn test_pin_set_high() {
    let report = tracing::log::Report::new();

    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();
    assert_eq!(report.get_logs().len(), 2);

    output.set_low();
    assert_eq!(report.get_logs().len(), 1);
}

#[test]
fn test_pin_set_high_print() {
    let _report = tracing::print::Report::new();

    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();

    output.set_low();
}

#[test]
fn test_without_test_with() {
    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();
}
