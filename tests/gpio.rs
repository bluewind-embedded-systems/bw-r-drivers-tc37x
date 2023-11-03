use tc37x_hal::tracing::print::redirect_to_print;

#[test]
fn test_pin_set_high() {
    redirect_to_print();

    use tc37x_hal::gpio::GpioExt;
    use tc37x_pac::PORT_00;
    let gpio00 = PORT_00.split();
    let mut output = gpio00.p00_5.into_push_pull_output();
    output.set_high();
}
