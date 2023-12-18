// TODO Remove asap
#![allow(dead_code)]

use super::baud_rate::*;
use super::can_module::{CanModuleId, ClockSource};
use super::frame::Frame;
use super::CanModule;
use crate::scu::wdt_call;
use crate::util::wait_nop_cycles;
use std::mem::transmute;
use tc37x_pac::hidden::RegValue;

// TODO Default values are not valid
#[derive(Default)]
pub struct BaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
}

// TODO Default values are not valid
#[derive(Default)]
pub struct FastBaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
    pub transceiver_delay_offset: u8,
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum FrameMode {
    // TODO refactor (annabo)
    #[default]
    Standard,
    FdLong,
    FdLongAndFast,
}
#[derive(PartialEq, Debug, Default)]
pub enum FrameType
// TODO refactor (annabo)
{
    #[default]
    Receive,
    Transmit,
    TransmitAndReceive,
    RemoteRequest,
    RemoteAnswer,
}

#[derive(Clone, Copy, Default)]
pub enum TxMode {
    #[default]
    DedicatedBuffers,
    Fifo,
    Queue,
    SharedFifo,
    SharedQueue,
}

#[derive(Clone, Copy, Default)]
pub enum RxMode {
    #[default]
    DedicatedBuffers,
    Fifo0,
    Fifo1,
    SharedFifo0,
    SharedFifo1,
    SharedAll,
}

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
    pub calculate_bit_timing_values: bool,
    pub baud_rate: BaudRate,
    pub fast_baud_rate: FastBaudRate,
    pub frame_mode: FrameMode,
    pub frame_type: FrameType,
    pub tx_mode: TxMode,
    pub rx_mode: RxMode,
    pub tx_buffer_data_field_size: u8, //(TODO) limit possibile values to valid ones
    pub message_ram_tx_buffers_start_address: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId(pub(crate) u8);

impl NodeId {
    pub const fn new(n: u8) -> Self {
        Self(n)
    }
}

pub struct NewCanNode {
    module: CanModule,
    node_id: NodeId,
    inner: tc37x_pac::can0::Node,
}

pub struct CanNode {
    module: CanModule,
    node_id: NodeId,
    inner: tc37x_pac::can0::Node,
    frame_mode: FrameMode,
}

impl CanNode {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: CanModule, node_id: NodeId) -> NewCanNode {
        let inner = module.registers().node(node_id.0.into());
        NewCanNode {
            module,
            node_id,
            inner,
        }
    }
}

impl NewCanNode {
    pub fn configure(self, config: CanNodeConfig) -> Result<CanNode, ()> {
        self.module
            .set_clock_source(self.node_id.into(), config.clock_source);

        // TODO Document why this is needed
        wait_nop_cycles(10);

        self.enable_configuration_change();

        self.configure_baud_rate(config.calculate_bit_timing_values, &config.baud_rate);

        // for CAN FD frames, set fast baud rate
        if config.frame_mode != FrameMode::Standard {
            self.configure_fast_baud_rate(
                config.calculate_bit_timing_values,
                &config.fast_baud_rate,
            );
        }

        // transmit frame configuration
        if let FrameType::Transmit
        | FrameType::TransmitAndReceive
        | FrameType::RemoteRequest
        | FrameType::RemoteAnswer = config.frame_type
        {
            self.set_tx_buffer_data_field_size(config.tx_buffer_data_field_size);
            self.set_tx_buffer_start_address(config.message_ram_tx_buffers_start_address);
        }

        self.set_frame_mode(config.frame_mode);

        self.disable_configuration_change();

        // TODO FifoData from config
        self.set_rx_fifo0(FifoData {
            field_size: DataFieldSize::_8,
            operation_mode: RxFifoMode::Blocking,
            watermark_level: 0,
            size: 4,
            start_address: 0x100,
        });

        // TODO DedicatedData from config
        self.set_tx_fifo(
            DedicatedData {
                field_size: DataFieldSize::_8,
                start_address: 0x440,
            },
            4,
        );

        self.set_interrupt(
            InterruptGroup::Rxf0n,
            Interrupt::RxFifo0newMessage,
            InterruptLine(1),
            2,
            Tos::Cpu0,
        );

        self.connect_pin_rx(
            RXD00B_P20_7_IN,
            InputMode::PULL_UP,
            PadDriver::CmosAutomotiveSpeed3,
        );

        // self.connect_pin_tx(
        //     TXD00_P20_8_OUT,
        //     OutputMode::PUSH_PULL,
        //     PadDriver::CmosAutomotiveSpeed3,
        // );

        Ok(CanNode {
            frame_mode: config.frame_mode,
            module: self.module,
            node_id: self.node_id,
            inner: self.inner,
        })
    }

    fn set_rx_fifo0(&self, data: FifoData) {
        self.set_rx_fifo0_data_field_size(data.field_size);
        self.set_rx_fifo0_start_address(data.start_address);
        self.set_rx_fifo0_size(data.size);
        self.set_rx_fifo0_operating_mode(data.operation_mode);
        self.set_rx_fifo0_watermark_level(data.watermark_level);
    }

    fn set_rx_fifo0_data_field_size(&self, size: DataFieldSize) {
        let size = tc37x_pac::can0::node::rxesc::F0Ds(size as u8);
        unsafe { self.inner.rxesc().modify(|r| r.f0ds().set(size)) };
    }

    fn set_rx_fifo0_start_address(&self, address: u16) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0sa().set(address >> 2)) };
    }

    fn set_rx_fifo0_size(&self, size: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0s().set(size)) };
    }

    fn set_rx_fifo0_watermark_level(&self, level: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0wm().set(level)) };
    }

    fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
        unsafe {
            self.inner
                .rxf0c()
                .modify(|r| r.f0om().set(mode == RxFifoMode::Overwrite))
        };
    }

    fn set_tx_fifo(&self, buffers: DedicatedData, fifo_size: u8) {
        self.set_inner_tx_buffers(buffers);
        self.set_inner_tx_fifo_queue(TxMode::Fifo, fifo_size);
        self.set_inner_tx_int(fifo_size);
    }

    fn set_inner_tx_buffers(&self, dedicated: DedicatedData) {
        self.set_tx_buffer_data_field_size(dedicated.field_size as u8);
        self.set_tx_buffer_start_address(dedicated.start_address);
    }

    fn set_inner_tx_fifo_queue(&self, mode: TxMode, size: u8) {
        self.set_transmit_fifo_queue_mode(mode);
        self.set_transmit_fifo_queue_size(size);
    }

    fn set_inner_tx_int(&self, size: u8) {
        for id in 0..size {
            self.enable_tx_buffer_transmission_interrupt(TxBufferId(id));
        }
    }

    fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
        unsafe {
            self.inner.txbtie().modify(|mut r| {
                *r.data_mut_ref() |= 1 << tx_buffer_id.0;
                r
            })
        };
    }

    fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
        if let TxMode::Fifo | TxMode::Queue = mode {
            let val = (mode as u8) != 0;
            unsafe { self.inner.txbc().modify(|r| r.tfqm().set(val)) };
        } else {
            panic!("invalid fifo queue mode");
        }
    }

    fn set_transmit_fifo_queue_size(&self, number: u8) {
        unsafe { self.inner.txbc().modify(|r| r.tfqs().set(number)) };
    }

    fn enable_configuration_change(&self) {
        let cccr = self.inner.cccr();

        if unsafe { cccr.read() }.init().get() {
            unsafe { cccr.modify(|r| r.cce().set(false)) };
            while unsafe { cccr.read() }.cce().get() {}

            unsafe { cccr.modify(|r| r.init().set(false)) };
            while unsafe { cccr.read() }.init().get() {}
        }

        unsafe { cccr.modify(|r| r.init().set(true)) };
        while !unsafe { cccr.read() }.init().get() {}

        unsafe { cccr.modify(|r| r.cce().set(true).init().set(true)) };
    }

    fn disable_configuration_change(&self) {
        let cccr = self.inner.cccr();

        unsafe { cccr.modify(|r| r.cce().set(false)) };

        while unsafe { cccr.read() }.cce().get() {}

        unsafe { cccr.modify(|r| r.init().set(false)) };

        while unsafe { cccr.read() }.init().get() {}
    }

    fn configure_baud_rate(&self, calculate_bit_timing_values: bool, baud_rate: &BaudRate) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            let timing: BitTiming = calculate_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
            self.set_bit_timing(timing);
        } else {
            self.set_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar,
            )
        }
    }

    fn configure_fast_baud_rate(
        &self,
        calculate_bit_timing_values: bool,
        baud_rate: &FastBaudRate,
    ) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            self.set_fast_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
        } else {
            self.set_fast_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar as u8,
            );
        }

        if baud_rate.transceiver_delay_offset != 0 {
            self.set_transceiver_delay_compensation_offset(baud_rate.transceiver_delay_offset);
        }
    }

    fn set_bit_timing(&self, timing: BitTiming) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nbrp()
                    .set(timing.brp)
                    .nsjw()
                    .set(timing.sjw)
                    .ntseg1()
                    .set(timing.tseg1)
                    .ntseg2()
                    .set(timing.tseg2)
            })
        }
    }

    fn set_bit_timing_values(&self, sjw: u8, time_segment2: u8, time_segment1: u8, prescaler: u16) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nsjw()
                    .set(sjw)
                    .ntseg1()
                    .set(time_segment1)
                    .ntseg2()
                    .set(time_segment2)
                    .nbrp()
                    .set(prescaler)
            })
        };
    }

    fn set_fast_bit_timing(&self, module_freq: f32, baudrate: u32, sample_point: u16, sjw: u16) {
        let timing = calculate_fast_bit_timing(module_freq, baudrate, sample_point, sjw);

        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dbrp()
                    .set(timing.brp.try_into().unwrap())
                    .dsjw()
                    .set(timing.sjw)
                    .dtseg1()
                    .set(timing.tseg1)
                    .dtseg2()
                    .set(timing.tseg2)
            })
        }
    }

    fn set_fast_bit_timing_values(
        &self,
        sjw: u8,
        time_segment2: u8,
        time_segment1: u8,
        prescaler: u8,
    ) {
        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dsjw()
                    .set(sjw)
                    .dtseg1()
                    .set(time_segment1)
                    .dtseg2()
                    .set(time_segment2)
                    .dbrp()
                    .set(prescaler)
            })
        };
    }

    fn set_tx_buffer_data_field_size(&self, data_field_size: u8) {
        let data_field_size = tc37x_pac::can0::node::txesc::Tbds(data_field_size);
        unsafe { self.inner.txesc().modify(|r| r.tbds().set(data_field_size)) };
    }

    fn set_tx_buffer_start_address(&self, address: u16) {
        unsafe { self.inner.txbc().modify(|r| r.tbsa().set(address >> 2)) };
    }

    fn set_frame_mode(&self, frame_mode: FrameMode) {
        let (fdoe, brse) = match frame_mode {
            FrameMode::Standard => (false, false),
            FrameMode::FdLong => (true, false),
            FrameMode::FdLongAndFast => (true, true),
        };

        unsafe {
            self.inner
                .cccr()
                .modify(|r| r.fdoe().set(fdoe).brse().set(brse))
        };
    }

    fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
        unsafe { self.inner.dbtp().modify(|r| r.tdc().set(true)) };
        unsafe { self.inner.tdcr().modify(|r| r.tdco().set(delay)) };
    }

    fn set_interrupt(
        &self,
        interrupt_group: InterruptGroup,
        interrupt: Interrupt,
        line: InterruptLine,
        priority: Priority,
        tos: Tos,
    ) {
        self.set_group_interrupt_line(interrupt_group, line);

        let src = tc37x_pac::SRC;

        use crate::pac::{src::Can0Int0, Reg, RW};

        let can_int: Reg<Can0Int0, RW> = match (self.module.id(), line) {
            // TODO Add other lines and can modules
            (CanModuleId::Can0, InterruptLine(0)) => unsafe { transmute(src.can0int0()) },
            (CanModuleId::Can0, InterruptLine(1)) => unsafe { transmute(src.can0int1()) },
            _ => unreachable!(),
        };

        let priority = priority.into();
        let tos = tos as u8;

        // Set priority and type of service
        unsafe { can_int.modify(|r| r.srpn().set(priority).tos().set(tos)) };

        // Clear request
        unsafe { can_int.modify(|r| r.clrr().set(true)) };

        // Enable service request
        unsafe { can_int.modify(|r| r.sre().set(true)) };

        // Enable interrupt
        unsafe {
            self.inner.ie().modify(|mut r| {
                *r.data_mut_ref() |= 1 << interrupt as u32;
                r
            })
        };
    }

    fn set_group_interrupt_line(
        &self,
        interrupt_group: InterruptGroup,
        interrupt_line: InterruptLine,
    ) {
        if interrupt_group <= InterruptGroup::Loi {
            unsafe {
                self.inner.grint1().modify(|mut r| {
                    *r.data_mut_ref() |= (interrupt_line.0 as u32) << (interrupt_group as u32 * 4);
                    r
                })
            };
        } else {
            unsafe {
                self.inner.grint2().modify(|mut r| {
                    *r.data_mut_ref() |=
                        (interrupt_line.0 as u32) << ((interrupt_group as u32 % 8) * 4);
                    r
                })
            };
        }
    }

    fn connect_pin_rx(&self, rxd: RxdIn, mode: InputMode, pad_driver: PadDriver) {
        let port = Port::new(rxd.port);
        port.set_pin_mode_input(rxd.pin_index, mode);
        port.set_pin_pad_driver(rxd.pin_index, pad_driver);

        unsafe {
            self.inner
                .npcr()
                .modify(|r| r.rxsel().set(rxd.select as u8))
        };
    }
}

impl CanNode {
    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }

    fn get_rx_fifo0_fill_level(&self) -> u8 {
        unsafe { self.inner.rxf0s().read() }.f0fl().get()
    }

    fn get_rx_fifo1_fill_level(&self) -> u8 {
        unsafe { self.inner.rxf1s().read() }.f1fl().get()
    }

    fn set_rx_buffers_start_address(&self, address: u16) {
        unsafe { self.inner.rxbc().modify(|r| r.rbsa().set(address >> 2)) };
    }

    fn set_rx_fifo0_size(&self, size: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0s().set(size)) };
    }

    fn set_rx_fifo0_start_address(&self, address: u16) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0sa().set(address >> 2)) };
    }

    fn set_rx_fifo0_watermark_level(&self, level: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0wm().set(level)) };
    }

    fn set_rx_fifo1_size(&self, size: u8) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1s().set(size)) };
    }

    fn set_rx_fifo1_start_address(&self, address: u16) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1sa().set(address >> 2)) };
    }

    fn set_rx_fifo1_watermark_level(&self, level: u8) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1wm().set(level)) };
    }

    fn is_tx_event_fifo_element_lost(&self) -> bool {
        unsafe { self.inner.txefs().read() }.tefl().get()
    }

    fn is_tx_event_fifo_full(&self) -> bool {
        unsafe { self.inner.txefs().read() }.eff().get()
    }

    fn is_tx_fifo_queue_full(&self) -> bool {
        unsafe { self.inner.txfqs().read() }.tfqf().get()
    }

    fn pause_trasmission(&self, enable: bool) {
        unsafe { self.inner.cccr().modify(|r| r.txp().set(enable)) };
    }

    fn set_dedicated_tx_buffers_number(&self, number: u8) {
        unsafe { self.inner.txbc().modify(|r| r.ndtb().set(number)) };
    }

    fn set_transmit_fifo_queue_size(&self, number: u8) {
        unsafe { self.inner.txbc().modify(|r| r.tfqs().set(number)) };
    }

    fn set_tx_event_fifo_start_address(&self, address: u16) {
        unsafe { self.inner.txefc().modify(|r| r.efsa().set(address >> 2)) };
    }

    fn set_tx_event_fifo_size(&self, size: u8) {
        unsafe { self.inner.txefc().modify(|r| r.efs().set(size)) };
    }

    fn set_standard_filter_list_start_address(&self, address: u16) {
        unsafe { self.inner.sidfc().modify(|r| r.flssa().set(address >> 2)) };
    }

    fn set_standard_filter_list_size(&self, size: u8) {
        unsafe { self.inner.sidfc().modify(|r| r.lss().set(size)) };
    }

    fn reject_remote_frames_with_standard_id(&self) {
        unsafe { self.inner.gfc().modify(|r| r.rrfs().set(true)) };
    }

    fn set_extended_filter_list_start_address(&self, address: u16) {
        unsafe { self.inner.xidfc().modify(|r| r.flesa().set(address >> 2)) };
    }

    fn set_extended_filter_list_size(&self, size: u8) {
        unsafe { self.inner.xidfc().modify(|r| r.lse().set(size)) };
    }

    fn reject_remote_frames_with_extended_id(&self) {
        unsafe { self.inner.gfc().modify(|r| r.rrfe().set(true)) };
    }
}

#[derive(Clone, Copy)]
pub struct FifoData {
    pub field_size: DataFieldSize,
    pub operation_mode: RxFifoMode,
    pub watermark_level: u8,
    pub size: u8,
    pub start_address: u16,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RxFifoMode {
    Blocking,
    Overwrite,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum DataFieldSize {
    _8,
    _12,
    _16,
    _20,
    _24,
    _32,
    _48,
    _64,
}

#[derive(Clone, Copy)]
pub struct DedicatedData {
    pub field_size: DataFieldSize,
    pub start_address: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InterruptGroup {
    Tefifo,
    Hpe,
    Wati,
    Alrt,
    Moer,
    Safe,
    Boff,
    Loi,
    Reint,
    Rxf1f,
    Rxf0f,
    Rxf1n,
    Rxf0n,
    Reti,
    Traq,
    Traco,
}

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    RxFifo0newMessage,
    RxFifo0watermarkReached,
    RxFifo0full,
    RxFifo0messageLost,
    RxFifo1newMessage,
    RxFifo1watermarkReached,
    RxFifo1full,
    RxFifo1messageLost,
    HighPriorityMessage,
    TransmissionCompleted,
    TransmissionCancellationFinished,
    TxFifoEmpty,
    TxEventFifoNewEntry,
    TxEventFifoWatermarkReached,
    TxEventFifoFull,
    TxEventFifoEventLost,
    TimestampWraparound,
    MessageRamaccessFailure,
    TimeoutOccurred,
    MessageStoredToDedicatedRxBuffer,
    BitErrorCorrected,
    BitErrorUncorrected,
    ErrorLoggingOverflow,
    ErrorPassive,
    WarningStatus,
    BusOffStatus,
    Watchdog,
    ProtocolErrorArbitration,
    ProtocolErrorData,
    AccessToReservedAddress,
}

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Default)]
pub struct InterruptLine(pub u8);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Tos {
    #[default]
    Cpu0,
    Dma,
    Cpu1,
    Cpu2,
}

const RXD00B_P20_7_IN: RxdIn =
    RxdIn::new(CanModuleId::Can0, NodeId(0), PortNumber::_20, 7, RxSel::_B);

const TXD00_P20_8_OUT: TxdOut = TxdOut::new(
    CanModuleId::Can0,
    NodeId(0),
    PortNumber::_20,
    8,
    OutputIdx::ALT5,
);

#[derive(Clone, Copy)]
pub struct InputMode(u32);
impl InputMode {
    pub const NO_PULL_DEVICE: Self = Self(0 << 3);
    pub const PULL_DOWN: Self = Self(1 << 3);
    pub const PULL_UP: Self = Self(2 << 3);
}

#[derive(Clone, Copy)]
pub struct OutputMode(u32);
impl OutputMode {
    pub const PUSH_PULL: OutputMode = Self(0x10 << 3);
    pub const OPEN_DRAIN: OutputMode = Self(0x18 << 3);
    pub const NONE: OutputMode = Self(0);
}

#[derive(Clone, Copy)]
pub enum PadDriver {
    CmosAutomotiveSpeed1 = 0,
    CmosAutomotiveSpeed2 = 1,
    CmosAutomotiveSpeed3 = 2,
    CmosAutomotiveSpeed4 = 3,
    TtlSpeed1 = 8,
    TtlSpeed2 = 9,
    TtlSpeed3 = 10,
    TtlSpeed4 = 11,
    Ttl3v3speed1 = 12,
    Ttl3v3speed2 = 13,
    Ttl3v3speed3 = 14,
    Ttl3v3speed4 = 15,
}

pub struct Port {
    inner: tc37x_pac::port_00::Port00,
}

#[derive(Clone, Copy)]
pub enum PortNumber {
    _00,
    _01,
    _02,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _20,
    _21,
    _22,
    _23,
    _32,
    _33,
    _34,
    _40,
}

#[derive(Debug, PartialEq)]
enum State {
    NotChanged = 0,
    High = 1,
    Low = 1 << 16,
    Toggled = (1 << 16) | 1,
}

impl Port {
    pub fn new(port: PortNumber) -> Self {
        use tc37x_pac::port_00::Port00;
        use tc37x_pac::*;

        let inner: Port00 = unsafe {
            match port {
                PortNumber::_00 => core::mem::transmute(PORT_00),
                PortNumber::_01 => core::mem::transmute(PORT_01),
                PortNumber::_02 => core::mem::transmute(PORT_02),
                PortNumber::_10 => core::mem::transmute(PORT_10),
                PortNumber::_11 => core::mem::transmute(PORT_11),
                PortNumber::_12 => core::mem::transmute(PORT_12),
                PortNumber::_13 => core::mem::transmute(PORT_13),
                PortNumber::_14 => core::mem::transmute(PORT_14),
                PortNumber::_15 => core::mem::transmute(PORT_15),
                PortNumber::_20 => core::mem::transmute(PORT_20),
                PortNumber::_21 => core::mem::transmute(PORT_21),
                PortNumber::_22 => core::mem::transmute(PORT_22),
                PortNumber::_23 => core::mem::transmute(PORT_23),
                PortNumber::_32 => core::mem::transmute(PORT_32),
                PortNumber::_33 => core::mem::transmute(PORT_33),
                PortNumber::_34 => core::mem::transmute(PORT_34),
                PortNumber::_40 => core::mem::transmute(PORT_40),
            }
        };
        Self { inner }
    }

    fn set_pin_state(&self, index: u8, action: State) {
        unsafe {
            self.inner.omr().init(|mut r| {
                let data = r.data_mut_ref();
                *data = ((action as u32) << index).into();
                r
            })
        };
    }

    fn toogle_pin(&self, index: u8) {
        self.set_pin_state(index, State::Toggled)
    }

    fn set_pin_high(&self, index: u8) {
        self.set_pin_state(index, State::High)
    }

    fn set_pin_low(&self, index: u8) {
        self.set_pin_state(index, State::Low)
    }

    fn set_pin_mode_output(&self, pin_index: u8, mode: OutputMode, index: OutputIdx) {
        self.set_pin_mode(pin_index, (mode, index).into());
    }

    fn set_pin_mode_input(&self, pin_index: u8, mode: InputMode) {
        self.set_pin_mode(pin_index, mode.into())
    }

    fn set_pin_mode(&self, index: u8, mode: Mode) {
        let ioc_index = index / 4;
        let shift = (index & 0x3) * 8;

        let is_supervisor =
            unsafe { transmute::<_, usize>(self.inner) == transmute(crate::pac::PORT_40) };
        if is_supervisor {
            wdt_call::call_without_cpu_endinit(|| unsafe {
                self.inner.pdisc().modify(|mut r| {
                    *r.data_mut_ref() &= !(1 << index);
                    r
                })
            });
        }

        let iocr: crate::pac::Reg<crate::pac::port_00::Iocr0, crate::pac::RW> = unsafe {
            let iocr0 = self.inner.iocr0();
            let addr: *mut u32 = transmute(iocr0);
            let addr = addr.add(ioc_index as _);
            transmute(addr)
        };

        unsafe {
            iocr.modify_atomic(|mut r| {
                *r.data_mut_ref() = (mode.0) << shift;
                *r.get_mask_mut_ref() = 0xFFu32 << shift;
                r
            })
        };
    }

    fn set_pin_pad_driver(&self, index: u8, driver: PadDriver) {
        let pdr_index = index / 8;
        let shift = (index & 0x7) * 4;
        let pdr: crate::pac::Reg<crate::pac::port_00::Pdr0, crate::pac::RW> = unsafe {
            let pdr0 = self.inner.pdr0();
            let addr: *mut u32 = core::mem::transmute(pdr0);
            let addr = addr.add(pdr_index as _);
            core::mem::transmute(addr)
        };

        wdt_call::call_without_cpu_endinit(|| unsafe {
            pdr.modify_atomic(|mut r| {
                *r.data_mut_ref() = (driver as u32) << shift;
                *r.get_mask_mut_ref() = 0xF << shift;
                r
            })
        });
    }
}

impl From<InputMode> for Mode {
    fn from(value: InputMode) -> Self {
        Mode(value.0)
    }
}

impl From<(OutputMode, OutputIdx)> for Mode {
    fn from(value: (OutputMode, OutputIdx)) -> Self {
        Mode(value.0 .0 | value.1 .0)
    }
}

struct Mode(u32);
impl Mode {
    const INPUT_NO_PULL_DEVICE: Mode = Self(0);
    const INPUT_PULL_DOWN: Mode = Self(8);
    const INPUT_PULL_UP: Mode = Self(0x10);
    const OUTPUT_PUSH_PULL_GENERAL: Mode = Self(0x80);
    const OUTPUT_PUSH_PULL_ALT1: Mode = Self(0x88);
    const OUTPUT_PUSH_PULL_ALT2: Mode = Self(0x90);
    const OUTPUT_PUSH_PULL_ALT3: Mode = Self(0x98);
    const OUTPUT_PUSH_PULL_ALT4: Mode = Self(0xA0);
    const OUTPUT_PUSH_PULL_ALT5: Mode = Self(0xA8);
    const OUTPUT_PUSH_PULL_ALT6: Mode = Self(0xB0);
    const OUTPUT_PUSH_PULL_ALT7: Mode = Self(0xB8);
    const OUTPUT_OPEN_DRAIN_GENERAL: Mode = Self(0xC0);
    const OUTPUT_OPEN_DRAIN_ALT1: Mode = Self(0xC8);
    const OUTPUT_OPEN_DRAIN_ALT2: Mode = Self(0xD0);
    const OUTPUT_OPEN_DRAIN_ALT3: Mode = Self(0xD8);
    const OUTPUT_OPEN_DRAIN_ALT4: Mode = Self(0xE0);
    const OUTPUT_OPEN_DRAIN_ALT5: Mode = Self(0xE8);
    const OUTPUT_OPEN_DRAIN_ALT6: Mode = Self(0xF0);
    const OUTPUT_OPEN_DRAIN_ALT7: Mode = Self(0xF8);
}

#[derive(Clone, Copy)]
struct OutputIdx(u32);
impl OutputIdx {
    const GENERAL: Self = Self(0x10 << 3);
    const ALT1: Self = Self(0x11 << 3);
    const ALT2: Self = Self(0x12 << 3);
    const ALT3: Self = Self(0x13 << 3);
    const ALT4: Self = Self(0x14 << 3);
    const ALT5: Self = Self(0x15 << 3);
    const ALT6: Self = Self(0x16 << 3);
    const ALT7: Self = Self(0x17 << 3);
}

#[derive(Clone, Copy)]
struct RxdIn {
    module: CanModuleId,
    node_id: NodeId,
    port: PortNumber,
    pin_index: u8,
    select: RxSel,
}

impl RxdIn {
    pub const fn new(
        module: CanModuleId,
        node_id: NodeId,
        port: PortNumber,
        pin_index: u8,
        select: RxSel,
    ) -> Self {
        Self {
            module,
            node_id,
            port,
            pin_index,
            select,
        }
    }
}

#[derive(Clone, Copy)]
pub enum RxSel {
    _A,
    _B,
    _C,
    _D,
    _E,
    _F,
    _G,
    _H,
}

#[derive(Clone, Copy)]
struct TxdOut {
    module: CanModuleId,
    node_id: NodeId,
    port: PortNumber,
    pin_index: u8,
    select: OutputIdx,
}

impl TxdOut {
    pub const fn new(
        module: CanModuleId,
        node_id: NodeId,
        port: PortNumber,
        pin_index: u8,
        select: OutputIdx,
    ) -> Self {
        Self {
            module,
            node_id,
            port,
            pin_index,
            select,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TxBufferId(pub u8);

pub type Priority = u8;
