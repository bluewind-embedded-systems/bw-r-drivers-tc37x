use tc37x_hal::gpio::GpioExt;
use tc37x_hal::tracing::log::reporter;
use tc37x_hal::tracing::test_with;
use tc37x_pac::PORT_00;

#[test]
fn test_pin_set_high() {
    let (reporter, report) = reporter();

    test_with(reporter, || {
        let gpio00 = PORT_00.split();
        let mut output = gpio00.p00_5.into_push_pull_output();
        output.set_high();
        assert_eq!(report.get_logs().len(), 2);
    });
}

#[test]
fn test_without_test_with() {
    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();

    output.set_high();
}
