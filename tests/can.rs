use tc37x_driver::tracing::log::Report;
use tc37x_driver::can::Module;
use tc37x_driver::can::Module0;

#[test]
fn test_can_module_enable(){
    let report = Report::new();
    let can_module = Module::new(Module0);

    // wtdcpu0con0 read
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_1);
    // wtdcpu0con0 write
    // wtdcpu0con0 read
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 write

    // clc read
    report.expect_read(0xF0208000, 4, 0b00000000000000000_00000000_0000_0_0_0);

    // wtdcpu0con0 read
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_1_0);

    // wtdcpu0con0 write
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_0_1);
    report.expect_read(0xF003624C, 4, 0b1111111111111100_00000000111100_0_1);



    let mut can_module = can_module.enable();

    insta::assert_snapshot!(report.take_log());
}
