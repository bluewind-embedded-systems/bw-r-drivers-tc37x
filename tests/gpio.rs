use tc37x_hal::gpio::GpioExt;
use tc37x_hal::tracing::print::PrintEffectReporter;
use tc37x_hal::tracing::test_with;
use tc37x_pac::PORT_00;

#[test]
fn test_pin_set_high() {
    test_with(
        || Box::new(PrintEffectReporter),
        || {
            let gpio00 = PORT_00.split();
            let mut output = gpio00.p00_5.into_push_pull_output();
            output.set_high();
        },
    );
}
