// TODO Remove asap
#![allow(dead_code)]

mod effects;

use super::baud_rate::*;
use super::can_module::{ClockSource, ModuleId};
use super::frame::{DataLenghtCode, Frame};
use super::internals::{Rx, Tx};
use super::msg::{ReadFrom, RxBufferId, TxBufferId};
use super::{can_module, Module};
use crate::can::msg::MessageId;

use crate::can::can_node::effects::NodeEffects;
use crate::log::{info, HexSlice};
use crate::scu::wdt_call;
use crate::util::wait_nop_cycles;
use core::mem::transmute;
use tc37x_pac::hidden::RegValue;
use tc37x_pac::RegisterValue;

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

#[derive(Clone, Copy, Default, PartialEq)]
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
pub struct NodeConfig {
    pub clock_source: ClockSource,
    pub baud_rate: BitTimingConfig,
    pub fast_baud_rate: FastBitTimingConfig,
    pub transceiver_delay_offset: u8,
    pub frame_mode: FrameMode,
    pub tx: Option<TxConfig>,
    pub rx_mode: RxMode,
    pub message_ram: MessageRAM,
}

#[derive(Clone, Copy)]
pub enum NodeId {
    Node0,
    Node1,
    Node2,
    Node3,
}

impl Into<u8> for NodeId {
    fn into(self) -> u8 {
        match self {
            NodeId::Node0 => 0,
            NodeId::Node1 => 1,
            NodeId::Node2 => 2,
            NodeId::Node3 => 3,
        }
    }
}

pub struct NewCanNode<N, M> {
    module: Module<M, can_module::Enabled>,
    node_id: NodeId,
    effects: NodeEffects<N>,
}

pub struct Node<N, M> {
    module: Module<M, can_module::Enabled>,
    node_id: NodeId,
    effects: NodeEffects<N>,
    frame_mode: FrameMode,
}

macro_rules! can_node {
    ($ModuleReg:ty, $NodeReg:path) => {

impl Node<$NodeReg, $ModuleReg> {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: Module<$ModuleReg, can_module::Enabled>, id: NodeId) -> NewCanNode<$NodeReg, $ModuleReg> {
        let node_index : u8 = id.into();
        let node_index : usize = node_index.into();
        let node_reg = module.registers().n()[node_index];
        let effects = NodeEffects::<$NodeReg>{reg:node_reg, node_id:id};
        NewCanNode {
            module,
            node_id:id,
            effects,
        }
    }
}

impl NewCanNode<$NodeReg, $ModuleReg> {
    pub fn configure(self, config: NodeConfig) -> Result<Node<$NodeReg, $ModuleReg>, ()> {
        self.module
            .set_clock_source(self.node_id.into(), config.clock_source)?;
        self.effects.enable_configuration_change();

        self.configure_baud_rate(&config.baud_rate);

        // for CAN FD frames, set fast baud rate
        if config.frame_mode != FrameMode::Standard {
            self.configure_fast_baud_rate(&config.fast_baud_rate);
        }

        // TODO Check if transceiver_delay_offset is needed only for CAN FD
        if config.transceiver_delay_offset != 0 {
            self.effects
                .set_transceiver_delay_compensation_offset(config.transceiver_delay_offset);
        }

        // transmit frame configuration
        if let Some(tx_config) = &config.tx {
            self.set_tx_buffer_data_field_size(tx_config.buffer_data_field_size);
            self.effects
                .set_tx_buffer_start_address(config.message_ram.tx_buffers_start_address);

            let mode = tx_config.mode;

            match mode {
                TxMode::DedicatedBuffers | TxMode::SharedFifo | TxMode::SharedQueue => {
                    self.effects
                        .set_dedicated_tx_buffers_number(tx_config.dedicated_tx_buffers_number);
                    if let TxMode::SharedFifo | TxMode::SharedQueue = mode {
                        if let TxMode::SharedFifo = mode {
                            self.set_transmit_fifo_queue_mode(TxMode::Fifo);
                        }
                        if let TxMode::SharedQueue = mode {
                            self.set_transmit_fifo_queue_mode(TxMode::Queue);
                        }
                        self.effects
                            .set_transmit_fifo_queue_size(tx_config.fifo_queue_size);
                    }
                    for id in 0..tx_config.dedicated_tx_buffers_number + tx_config.fifo_queue_size {
                        self.effects
                            .enable_tx_buffer_transmission_interrupt(TxBufferId(id));
                    }
                }
                TxMode::Fifo | TxMode::Queue => {
                    self.set_transmit_fifo_queue_mode(mode);
                    self.effects
                        .set_transmit_fifo_queue_size(tx_config.fifo_queue_size);
                    for id in 0..tx_config.fifo_queue_size {
                        self.effects
                            .enable_tx_buffer_transmission_interrupt(TxBufferId(id));
                    }
                }
            }

            if (1..=32).contains(&tx_config.event_fifo_size) {
                self.effects.set_tx_event_fifo_start_address(
                    config.message_ram.tx_event_fifo_start_address,
                );
                self.effects
                    .set_tx_event_fifo_size(tx_config.event_fifo_size);
            } else {
                crate::log::error!("Invalid event fifo size: {}", tx_config.event_fifo_size);
            }

            self.set_frame_mode(config.frame_mode);
        }

        self.effects.disable_configuration_change();

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

        // TODO Interrupt from config
        self.set_interrupt(
            InterruptGroup::Rxf0n,
            Interrupt::RxFifo0newMessage,
            InterruptLine(1),
            2,
            Tos::Cpu0,
        );

        // TODO Connect pins from config
        self.connect_pin_rx(
            RXD00B_P20_7_IN,
            InputMode::PULL_UP,
            PadDriver::CmosAutomotiveSpeed3,
        );

        // TODO Connect pins from config
        self.connect_pin_tx(
            TXD00_P20_8_OUT,
            OutputMode::PUSH_PULL,
            PadDriver::CmosAutomotiveSpeed3,
        );

        Ok(Node {
            frame_mode: config.frame_mode,
            module: self.module,
            node_id: self.node_id,
            effects: self.effects,
        })
    }

    fn set_rx_fifo0(&self, data: FifoData) {
        // TODO impl conversion DataFieldSize to u8
        self.effects.set_rx_fifo0_data_field_size(data.field_size.into_register_value());
        self.effects.set_rx_fifo0_start_address(data.start_address);
        self.effects.set_rx_fifo0_size(data.size);
        self.effects
            .set_rx_fifo0_operating_mode(data.operation_mode);
        self.effects
            .set_rx_fifo0_watermark_level(data.watermark_level);
    }

    fn set_tx_fifo(&self, buffers: DedicatedData, fifo_size: u8) {
        self.set_inner_tx_buffers(buffers);
        self.set_inner_tx_fifo_queue(TxMode::Fifo, fifo_size);
        self.set_inner_tx_int(fifo_size);
    }

    fn set_inner_tx_buffers(&self, dedicated: DedicatedData) {
        self.set_tx_buffer_data_field_size(dedicated.field_size);
        self.effects
            .set_tx_buffer_start_address(dedicated.start_address);
    }

    fn set_inner_tx_fifo_queue(&self, mode: TxMode, size: u8) {
        self.effects.set_transmit_fifo_queue_mode(mode);
        self.effects.set_transmit_fifo_queue_size(size);
    }

    fn set_inner_tx_int(&self, size: u8) {
        for id in 0..size {
            self.effects
                .enable_tx_buffer_transmission_interrupt(TxBufferId(id));
        }
    }

    fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
        if let TxMode::Fifo | TxMode::Queue = mode {
            self.effects.set_transmit_fifo_queue_mode(mode);
        } else {
            // TODO Avoid panic
            panic!("invalid fifo queue mode");
        }
    }

    fn configure_baud_rate(&self, baud_rate: &BitTimingConfig) {
        let bit_timing: NominalBitTiming = match baud_rate {
            BitTimingConfig::Auto(baud_rate) => {
                let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
                calculate_bit_timing(
                    module_freq,
                    baud_rate.baud_rate,
                    baud_rate.sample_point,
                    baud_rate.sync_jump_width,
                )
            }
            BitTimingConfig::Manual(baud_rate) => *baud_rate,
        };

        self.effects.set_nominal_bit_timing(&bit_timing);
    }

    fn configure_fast_baud_rate(&self, baud_rate: &FastBitTimingConfig) {
        let bit_timing: DataBitTiming = match baud_rate {
            FastBitTimingConfig::Auto(baud_rate) => {
                let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
                calculate_fast_bit_timing(
                    module_freq,
                    baud_rate.baud_rate,
                    baud_rate.sample_point,
                    baud_rate.sync_jump_width,
                )
            }
            FastBitTimingConfig::Manual(baud_rate) => *baud_rate,
        };

        self.effects.set_data_bit_timing(&bit_timing);
    }

    #[inline]
    pub fn set_tx_buffer_data_field_size(&self, data_field_size: DataFieldSize) {
        info!("Data field size: {}", data_field_size.into_register_value());
        self.effects.set_tx_buffer_data_field_size(data_field_size.into_register_value());
    }

    fn set_frame_mode(&self, frame_mode: FrameMode) {
        let (fdoe, brse) = match frame_mode {
            FrameMode::Standard => (false, false),
            FrameMode::FdLong => (true, false),
            FrameMode::FdLongAndFast => (true, true),
        };

        self.effects.set_frame_mode(fdoe, brse);
    }

    // FIXME Fix set_interrupt. Broken after update of pac (SRC is missing)
    fn set_interrupt(
        &self,
        interrupt_group: InterruptGroup,
        interrupt: Interrupt,
        line: InterruptLine,
        _priority: Priority,
        _tos: Tos,
    ) {
        self.set_group_interrupt_line(interrupt_group, line);

        // let src = tc37x_pac::SRC;
        //
        // use crate::pac::{Reg, RW};
        //
        // let can_int: Reg<Can0Int0, RW> = match (self.module.id(), line) {
        //     // TODO Add other lines and can modules
        //     (ModuleId::Can0, InterruptLine(0)) => unsafe { transmute(src.can0int0()) },
        //     (ModuleId::Can0, InterruptLine(1)) => unsafe { transmute(src.can0int1()) },
        //     (ModuleId::Can1, InterruptLine(0)) => unsafe { transmute(src.can1int0()) },
        //     (ModuleId::Can1, InterruptLine(1)) => unsafe { transmute(src.can1int1()) },
        //     _ => unreachable!(),
        // };
        //
        // let priority = priority;
        // let tos = tos as u8;

        // Set priority and type of service
        // unsafe { can_int.modify(|r| r.srpn().set(priority).tos().set(tos)) };

        // Clear request
        // unsafe { can_int.modify(|r| r.clrr().set(true)) };

        // Enable service request
        // unsafe { can_int.modify(|r| r.sre().set(true)) };

        // Enable interrupt
        self.effects.enable_interrupt(interrupt);
    }

    fn set_group_interrupt_line(
        &self,
        interrupt_group: InterruptGroup,
        interrupt_line: InterruptLine,
    ) {
        let line = interrupt_line.0 as u32;

        if interrupt_group <= InterruptGroup::Loi {
            let group = interrupt_group as u32 * 4;
            self.effects.set_interrupt_routing_group_1(line, group);
        } else {
            let group = (interrupt_group as u32 % 8) * 4;
            self.effects.set_interrupt_routing_group_2(line, group);
        }
    }

    fn connect_pin_rx(&self, rxd: RxdIn, mode: InputMode, pad_driver: PadDriver) {
        let port = Port::new(rxd.port);
        port.set_pin_mode_input(rxd.pin_index, mode);
        port.set_pin_pad_driver(rxd.pin_index, pad_driver);
        self.effects.connect_pin_rx(rxd.select);
    }

    fn connect_pin_tx(&self, txd: TxdOut, mode: OutputMode, pad_driver: PadDriver) {
        let port = Port::new(txd.port);
        port.set_pin_mode_output(txd.pin_index, mode, txd.select);
        port.set_pin_pad_driver(txd.pin_index, pad_driver);
        port.set_pin_low(txd.pin_index);
    }
}

impl Node<$NodeReg, $ModuleReg> {
    pub fn transmit(&self, frame: &Frame) -> Result<(), ()> {
        let buffer_id = self.get_tx_fifo_queue_put_index();
        self.transmit_inner(buffer_id, frame.id, false, false, false, frame.data)
    }

    pub fn get_tx_fifo_queue_put_index(&self) -> TxBufferId {
        let id = self.effects.get_tx_fifo_queue_put_index();
        TxBufferId::new_const(id)
    }

    // TODO Use a meaningful error type
    #[allow(unused_variables)]
    fn transmit_inner(
        &self,
        buffer_id: TxBufferId,
        id: MessageId,
        tx_event_fifo_control: bool,
        remote_transmit_request: bool,
        error_state_indicator: bool,
        data: &[u8],
    ) -> Result<(), ()> {
        info!("transmit_inner");

        // TODO list errors
        if self.effects.is_tx_buffer_request_pending(buffer_id) {
            return Err(());
        }

        // FIXME Use real base address
        //let ram_base_address = 0xF0200000u32;
        let ram_base_address = self.module.ram_base_address() as u32;

        // FIXME Use real start address
        let tx_buffers_start_address = 0x0440u16;

        let tx_buf_el =
            self.get_tx_element_address(ram_base_address, tx_buffers_start_address, buffer_id);

        tx_buf_el.set_msg_id(id);

        if tx_event_fifo_control {
            tx_buf_el.set_tx_event_fifo_ctrl(tx_event_fifo_control);
            tx_buf_el.set_message_marker(buffer_id);
        }

        tx_buf_el.set_remote_transmit_req(remote_transmit_request);

        if let FrameMode::FdLong | FrameMode::FdLongAndFast = self.frame_mode {
            tx_buf_el.set_err_state_indicator(error_state_indicator)
        }

        let data_len: u8 = data.len().try_into().map_err(|_| ())?;
        let dlc = DataLenghtCode::try_from(data_len)?;

        tx_buf_el.set_data_length(dlc);
        tx_buf_el.write_tx_buf_data(dlc, data.as_ptr());
        tx_buf_el.set_frame_mode_req(self.frame_mode);
        self.effects.set_tx_buffer_add_request(/*buffer_id*/);

        info!("transmit {}#{}", id.data, HexSlice::from(data));

        Ok(())
    }
}

// IfxLld_Can_Std_Rx_Element_Functions
impl Node<$NodeReg, $ModuleReg> {
    pub fn get_rx_fifo0_get_index(&self) -> RxBufferId {
        let id = self.effects.get_rx_fifo0_get_index();
        RxBufferId::new_const(id)
    }

    pub fn get_rx_fifo1_get_index(&self) -> RxBufferId {
        let id = self.effects.get_rx_fifo1_get_index();
        RxBufferId::new_const(id)
    }

    #[inline]
    pub fn set_rx_buffer_data_field_size(&self, size: DataFieldSize) {
        self.effects.set_rx_buffer_data_field_size(size.into_register_value());
    }

    pub fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: RxBufferId) -> bool {
        self.effects.is_rx_buffer_new_data_updated(rx_buffer_id.0)
    }
}

// IfxLld_Can_Std_Tx_Element_Functions
impl Node<$NodeReg, $ModuleReg> {
    #[inline]
    pub fn is_tx_buffer_cancellation_finished(&self, tx_buffer_id: TxBufferId) -> bool {
        self.is_tx_buffer_transmission_occured(tx_buffer_id)
    }

    #[inline]
    pub fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: TxBufferId) -> bool {
        self.effects
            .is_tx_buffer_transmission_occured(tx_buffer_id.0)
    }

    pub fn get_rx_element_address(
        &self,
        ram_base_address: u32,
        tx_buffers_start_address: u16,
        buf_from: ReadFrom,
        buffer_number: RxBufferId,
    ) -> Rx {
        let num_of_config_bytes = 8u32;
        let num_of_data_bytes = self.effects.get_data_field_size(buf_from) as u32;
        let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
        let tx_buffer_index = tx_buffer_size * u32::from(buffer_number);

        let tx_buffer_element_address =
            ram_base_address + tx_buffers_start_address as u32 + tx_buffer_index;

        Rx::new(tx_buffer_element_address as *mut u8)
    }

    pub fn get_tx_element_address(
        &self,
        ram_base_address: u32,
        tx_buffers_start_address: u16,
        buffer_number: TxBufferId,
    ) -> Tx {
        let num_of_config_bytes = 8u32;
        let num_of_data_bytes = self.effects.get_tx_buffer_data_field_size() as u32;
        let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
        let tx_buffer_index = tx_buffer_size * u32::from(buffer_number);

        let tx_buffer_element_address =
            ram_base_address + tx_buffers_start_address as u32 + tx_buffer_index;

        Tx::new(tx_buffer_element_address as *mut u8)
    }
}
    }
}

can_node!(crate::pac::can0::Can0, crate::pac::can0::N);
can_node!(crate::pac::can1::Can1, crate::pac::can1::N);

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
#[derive(Clone, Copy, Default)]
pub enum DataFieldSize {
    #[default]
    _8,
    _12,
    _16,
    _20,
    _24,
    _32,
    _48,
    _64,
}

impl DataFieldSize {
    fn into_register_value(self) -> u8 {
        let value = match self {
            DataFieldSize::_8 => 0,
            DataFieldSize::_12 => 1,
            DataFieldSize::_16 => 2,
            DataFieldSize::_20 => 3,
            DataFieldSize::_24 => 4,
            DataFieldSize::_32 => 5,
            DataFieldSize::_48 => 6,
            DataFieldSize::_64 => 7,
        };
        value
    }
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
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_20, 7, RxSel::_B);

const TXD00_P20_8_OUT: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_20,
    8,
    OutputIdx::ALT5,
);

const TXD00_P20_6_OUT: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_20,
    6,
    OutputIdx::GENERAL,
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
                *data = (action as u32) << index;
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
    module: ModuleId,
    node_id: NodeId,
    port: PortNumber,
    pin_index: u8,
    select: RxSel,
}

impl RxdIn {
    pub const fn new(
        module: ModuleId,
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
    module: ModuleId,
    node_id: NodeId,
    port: PortNumber,
    pin_index: u8,
    select: OutputIdx,
}

impl TxdOut {
    pub const fn new(
        module: ModuleId,
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

pub type Priority = u8;

#[derive(Clone, Copy)]
pub struct TxConfig {
    pub mode: TxMode,
    pub dedicated_tx_buffers_number: u8,
    pub fifo_queue_size: u8,
    pub buffer_data_field_size: DataFieldSize,
    pub event_fifo_size: u8,
}

#[derive(Clone, Copy)]
pub struct MessageRAM {
    pub standard_filter_list_start_address: u16,
    pub extended_filter_list_start_address: u16,
    pub rx_fifo0_start_address: u16,
    pub rx_fifo1_start_address: u16,
    pub rx_buffers_start_address: u16,
    pub tx_event_fifo_start_address: u16,
    pub tx_buffers_start_address: u16,
}

impl Default for MessageRAM {
    fn default() -> Self {
        Self {
            standard_filter_list_start_address: 0x0,
            extended_filter_list_start_address: 0x80,
            rx_fifo0_start_address: 0x100,
            rx_fifo1_start_address: 0x200,
            rx_buffers_start_address: 0x300,
            tx_event_fifo_start_address: 0x400,
            tx_buffers_start_address: 0x440,
        }
    }
}
