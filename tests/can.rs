use tc37x_driver::tracing::log::Report;
use tc37x_driver::can::{
    Module,
    Module0,
    NodeConfig,
    BitTimingConfig,
    AutoBitTiming,
    Node0,
    TxConfig,
    TxMode,
    DataFieldSize,
    RxConfig,
    RxMode,
    RxFifoMode,
    Pins,
    pin_map::{PIN_TX_0_0_P20_8, PIN_RX_0_0_P20_7},

};
use tc37x as pac;
use pac::{CAN0, SCU};

// TODO fix values of can_module.enable reads
// TODO add report comments with actual registers' name
#[test]
fn test_can_module_take_node(){
    let report = Report::new();
    let can_module = Module::new(Module0);

    // wtdcpu0con0 for clear_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    // wtdcpu0con0 write
    // wtdcpu0con0 read
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 write

    // clc read
    report.expect_read(CAN0.clc().addr(), 4, 0b0);

    // wtdcpu0con0 for set_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);

    // wtdcpu0con0 write
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);

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
    report.expect_read(CAN0.mcr().addr(), 4, 0b0);
    report.expect_read(CAN0.mcr().addr(), 4, 0b11);

    // cccr for enable configuration change
    // read
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);
    
    // modify
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);

    // read
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);
    
    // modify
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);
    
    // read
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b0);
    
    // modify
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b0);
    
    // read
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);

    // modify
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b1);

    // ccucon1 for get_mcan_frequency for configure_baud_rate
    report.expect_read(SCU.ccucon1().addr(), 4, 0b0010_0001_0001_0001_0000_0010_0001_0010);

    // ccucon0 read for get_source_frequency for get_mcan_frequency
    report.expect_read(SCU.ccucon0().addr(), 4, 0b0001_0111_0010_0011_0000_0001_0001_0011);

    // syspllcon0 read for get_osc_frequency for get_per_pll_frequency1 for get_source_frequency
    report.expect_read(SCU.syspllcon0().addr(), 4, 0b0100_0000_0000_0001_0011_1010_0000_0000);

    // perpllcon0 read for get_per_pll_frequency1 for get_source_frequency
    report.expect_read(SCU.perpllcon0().addr(), 4, 0b10011_1111_0000_0000);

    // perpllcon1 read for get_per_pll_frequency1 for get_source_frequency
    report.expect_read(SCU.perpllcon1().addr(), 4, 0b1_0000_0001);

    // ccucon1 for get_source_frequency
    report.expect_read(SCU.ccucon1().addr(), 4, 0b10_0001_0001_0001_0000_0010_0001_0010);

    // nbtp0 for set_nominal_bit_timing
    report.expect_read(CAN0.n()[0].nbtpi().addr(), 4, 0b110_0000_0000_0000_1010_0000_0011);

    let mut node = can_module.take_node(Node0, cfg).expect("Cannot take can node");

    // txesc0 for set_tx_buffer_data_field_size for setup_tx
    report.expect_read(CAN0.n()[0].tx().txesci().addr(), 4, 0b0);

    // txbc0 for set_tx_buffer_start_address for setup_tx
    report.expect_read(CAN0.n()[0].tx().txbci().addr(), 4, 0b0);

    // txbc0 for set_dedicated_tx_buffers_number for setup_tx
    report.expect_read(CAN0.n()[0].tx().txbci().addr(), 4, 0b100_0100_0000);

    // txbtie0 for enable_tx_buffer_transmission_interrupt for setup_tx
    report.expect_read(CAN0.n()[0].tx().txbtiei().addr(), 4, 0b0);
    report.expect_read(CAN0.n()[0].tx().txbtiei().addr(), 4, 0b1);

    // txefc0 for set_tx_event_fifo_start_address for setup_tx
    report.expect_read(CAN0.n()[0].tx().txefci().addr(), 4, 0b0);
    // txefc0 for set_tx_event_fifo_size for setup_tx
    report.expect_read(CAN0.n()[0].tx().txefci().addr(), 4, 0b100_0000_0000);

    // cccr0 for set_frame_mode for setup_tx
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b11);

    node.setup_tx(&TxConfig {
        mode: TxMode::DedicatedBuffers,
        dedicated_tx_buffers_number: 2,
        fifo_queue_size: 0,
        buffer_data_field_size: DataFieldSize::_8,
        event_fifo_size: 1,
        tx_event_fifo_start_address: 0x400,
        tx_buffers_start_address: 0x440,
    });

    // rxesc0 for set_rx_buffer_data_field_size for setup_rx
    report.expect_read(CAN0.n()[0].rx().rxesci().addr(), 4, 0b0);

    // rxbc0 for set_rx_buffer_start_address for setup_rx
    report.expect_read(CAN0.n()[0].rx().rxbci().addr(), 4, 0b0);

    // rxesc0 for set_rx_fifo0_data_field_size for setup_rx
    report.expect_read(CAN0.n()[0].rx().rxesci().addr(), 4, 0b0);

    // rxf0c0 for set_rx_fifo0_start_address
    report.expect_read(CAN0.n()[0].rx().rxf0ci().addr(), 4, 0b0);

    // rxf0c0 for set_rx_fifo0_size
    report.expect_read(CAN0.n()[0].rx().rxf0ci().addr(), 4, 0b1_0000_0000);

    // rxf0c0 for set_rx_fifo0_operating_mode
    report.expect_read(CAN0.n()[0].rx().rxf0ci().addr(), 4, 0b0100_0000_0001_0000_0000);

    // rxf0c0 for set_rx_fifo0_watermark_level
    report.expect_read(CAN0.n()[0].rx().rxf0ci().addr(), 4, 0b0100_0000_0001_0000_0000);

    // cccr0 for set_frame_mode for setup_tx
    report.expect_read(CAN0.n()[0].cccri().addr(), 4, 0b11);


    node.setup_rx(RxConfig {
        mode: RxMode::SharedFifo0,
        buffer_data_field_size: DataFieldSize::_8,
        fifo0_data_field_size: DataFieldSize::_8,
        fifo1_data_field_size: DataFieldSize::_8,
        fifo0_operating_mode: RxFifoMode::Blocking,
        fifo1_operating_mode: RxFifoMode::Blocking,
        fifo0_watermark_level: 0,
        fifo1_watermark_level: 0,
        fifo0_size: 4,
        fifo1_size: 0,
        rx_fifo0_start_address: 0x100,
        rx_fifo1_start_address: 0x200,
        rx_buffers_start_address: 0x300,
    });

    // wtdcpu0con0 for clear_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    // wtdcpu0con0 write
    // wtdcpu0con0 read
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 for set_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);

    // wtdcpu0con0 write
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 write
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);

    report.expect_read(CAN0.n()[0].npcri().addr(), 4, 0b0);

    // wtdcpu0con0 for clear_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);
    // wtdcpu0con0 write
    // wtdcpu0con0 read
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 for set_cpu_endinit
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_0);

    // wtdcpu0con0 write
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_0_1);

    // wtdcpu0con0 write
    report.expect_read(SCU.wdtcpu0con0().addr(), 4, 0b1111111111111100_00000000111100_1_1);

    node.setup_pins(&Pins {
        tx: PIN_TX_0_0_P20_8,
        rx: PIN_RX_0_0_P20_7,
    });

    insta::assert_snapshot!(report.take_log());
}
