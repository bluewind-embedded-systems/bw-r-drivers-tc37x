use tc37x_driver::tracing::log::Report;
use tc37x_driver::can::{Module, Module0, NodeConfig, BitTimingConfig, AutoBitTiming, Node0};

#[test]
fn test_can_module_take_node(){
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

    let cfg = NodeConfig {
        baud_rate: BitTimingConfig::Auto(AutoBitTiming {
            baud_rate: 1_000_000,
            sample_point: 8_000,
            sync_jump_width: 3,
        }),
        ..Default::default()
    };

    // mcr read for set clock source
    report.expect_read(0xF0208030, 4, 0x0000);
    report.expect_read(0xF0208030, 4, 0b0_0_0_0_0_000_0000000000000000_00_00_00_11);

    // cccr for enable configuration change
    // read
    report.expect_read(0xF0208218, 4, 0b1);
    
    // modify
    report.expect_read(0xF0208218, 4, 0b1);

    // read
    report.expect_read(0xF0208218, 4, 0b1);
    
    // modify
    report.expect_read(0xF0208218, 4, 0b1);
    
    // read
    report.expect_read(0xF0208218, 4, 0b0);
    
    // modify
    report.expect_read(0xF0208218, 4, 0b0);
    
    // read
    report.expect_read(0xF0208218, 4, 0b1);

    // modify
    report.expect_read(0xF0208218, 4, 0b1);

    // ccucon1 read for get_mcan_frequency for configure_baud_rate
    // ccucon0 read for get_source_frequency for get_mcan_frequency


    let mut node = can_module.take_node(Node0, cfg).expect("Cannot take can node");

    insta::assert_snapshot!(report.take_log());
}
