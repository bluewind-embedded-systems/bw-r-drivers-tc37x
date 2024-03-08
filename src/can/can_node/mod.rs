// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]
// TODO Remove asap
#![allow(dead_code)]

pub mod config;
mod effects;

use super::baud_rate::*;
use super::frame::{DataLenghtCode, Frame};
use super::internals::Tx;
use super::msg::TxBufferId;
use super::{can_module, Module, ModuleId};
use crate::can::can_module::ClockSelect;
use crate::can::can_node::effects::NodeEffects;
use crate::can::config::NodeInterruptConfig;
use crate::can::msg::FrameMode;
use crate::can::msg::MessageId;
use crate::can::msg::ReadFrom;
use crate::can::msg::RxMessage;
use crate::cpu::Priority;
use crate::log::info;
use crate::pac::common::RegisterValue;
use crate::scu::wdt_call;
pub use config::NodeConfig;
use core::marker::PhantomData;
use core::mem::transmute;

#[derive(PartialEq, Debug, Default)]
pub enum FrameType {
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

// TODO Suspicious
const TX_BUFFER_START_ADDRESS: u32 = 0x0440u32;

pub trait NodeId {
    const INDEX: usize;

    fn as_index(&self) -> usize {
        Self::INDEX
    }
}

pub struct Node0;
pub struct Node1;
pub struct Node2;
pub struct Node3;

impl NodeId for Node0 {
    const INDEX: usize = 0;
}
impl NodeId for Node1 {
    const INDEX: usize = 1;
}
impl NodeId for Node2 {
    const INDEX: usize = 2;
}
impl NodeId for Node3 {
    const INDEX: usize = 3;
}

// Type state of Node
pub struct Configured;
pub struct Configurable;

pub struct Node<N, M, I: NodeId, State> {
    effects: NodeEffects<N>,
    frame_mode: FrameMode,
    ram_base_address: u32,
    _phantom: PhantomData<(M, I, State)>,

    // TODO Not all of rx_config fields are used, keep only the ones that are used
    rx_config: Option<RxConfig>,
}

pub enum ConfigError {
    CannotSetClockSource,
}

pub enum TransmitError {
    Busy,
    InvalidDataLength,
    InvalidAccess,
}

macro_rules! impl_can_node {
    ($ModuleReg:ty, $NodeReg:path, $ModuleId: ty) => {
        // Methods only valid on a configurable node
        impl<I: NodeId> Node<$NodeReg, $ModuleReg, I, Configurable> {
            /// Only a module can create a self. This function is only accessible from within this crate.
            pub(super) fn new(
                module: &mut Module<$ModuleId, $ModuleReg, can_module::Enabled>,
                node_id: I,
                config: NodeConfig,
            ) -> Result<Node<$NodeReg, $ModuleReg, I, Configurable>, ConfigError> {
                let node_index = node_id.as_index();
                #[allow(clippy::indexing_slicing)]
                let node_reg = module.registers().n()[node_index];
                let effects = NodeEffects::<$NodeReg>::new(node_reg);
                let clock_select = ClockSelect::from(node_id);

                module
                    .set_clock_source(clock_select, config.clock_source)
                    .map_err(|_| ConfigError::CannotSetClockSource)?;

                let node = Self {
                    effects,
                    _phantom: PhantomData,
                    frame_mode: config.frame_mode,
                    ram_base_address: module.ram_base_address(),
                    rx_config: None,
                };

                node.effects.enable_configuration_change();

                node.configure_baud_rate(&config.baud_rate);

                // for CAN FD frames, set fast baud rate
                if config.frame_mode != FrameMode::Standard {
                    node.configure_fast_baud_rate(&config.fast_baud_rate);
                }

                // TODO Check if transceiver_delay_offset is needed only for CAN FD
                if config.transceiver_delay_offset != 0 {
                    node.effects
                        .set_transceiver_delay_compensation_offset(config.transceiver_delay_offset);
                }

                Ok(node)
            }

            pub fn lock_configuration(self) -> Node<$NodeReg, $ModuleReg, I, Configured> {
                self.effects.disable_configuration_change();

                Node {
                    effects: self.effects,
                    _phantom: PhantomData,
                    frame_mode: self.frame_mode,
                    ram_base_address: self.ram_base_address,
                    rx_config: self.rx_config,
                }
            }

            pub fn setup_tx(&self, tx_config: &TxConfig) {
                self.set_tx_buffer_data_field_size(tx_config.buffer_data_field_size);
                self.effects
                    .set_tx_buffer_start_address(tx_config.tx_buffers_start_address);

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
                        for id in
                            0..tx_config.dedicated_tx_buffers_number + tx_config.fifo_queue_size
                        {
                            if let Ok(tx_buffer_id) = TxBufferId::try_from(id) {
                                self.effects
                                    .enable_tx_buffer_transmission_interrupt(tx_buffer_id);
                            }
                        }
                    }
                    TxMode::Fifo | TxMode::Queue => {
                        self.set_transmit_fifo_queue_mode(mode);
                        self.effects
                            .set_transmit_fifo_queue_size(tx_config.fifo_queue_size);
                        for id in 0..tx_config.fifo_queue_size {
                            if let Ok(tx_buffer_id) = TxBufferId::try_from(id) {
                                self.effects
                                    .enable_tx_buffer_transmission_interrupt(tx_buffer_id);
                            }
                        }
                    }
                }

                if (1..=32).contains(&tx_config.event_fifo_size) {
                    self.effects
                        .set_tx_event_fifo_start_address(tx_config.tx_event_fifo_start_address);
                    self.effects
                        .set_tx_event_fifo_size(tx_config.event_fifo_size);
                } else {
                    crate::log::error!("Invalid event fifo size: {}", tx_config.event_fifo_size);
                }

                self.set_frame_mode(self.frame_mode);
            }

            pub fn setup_rx(&mut self, rx_config: RxConfig) {
                self.rx_config = Some(rx_config);

                let mode = rx_config.mode;

                match mode {
                    RxMode::DedicatedBuffers
                    | RxMode::SharedFifo0
                    | RxMode::SharedFifo1
                    | RxMode::SharedAll => {
                        self.set_rx_buffer_data_field_size(rx_config.buffer_data_field_size);
                        self.effects
                            .set_rx_buffer_start_address(rx_config.rx_buffers_start_address);

                        if let RxMode::SharedFifo0 | RxMode::SharedAll = mode {
                            self.set_rx_fifo0(FifoData {
                                field_size: rx_config.fifo0_data_field_size,
                                operation_mode: rx_config.fifo0_operating_mode,
                                watermark_level: rx_config.fifo0_watermark_level,
                                size: rx_config.fifo0_size,
                                start_address: rx_config.rx_fifo0_start_address,
                            });
                        }
                        if let RxMode::SharedFifo1 | RxMode::SharedAll = mode {
                            self.set_rx_fifo1(FifoData {
                                field_size: rx_config.fifo1_data_field_size,
                                operation_mode: rx_config.fifo1_operating_mode,
                                watermark_level: rx_config.fifo1_watermark_level,
                                size: rx_config.fifo1_size,
                                start_address: rx_config.rx_fifo1_start_address,
                            });
                        }
                    }
                    RxMode::Fifo0 => {
                        self.set_rx_fifo0(FifoData {
                            field_size: rx_config.fifo0_data_field_size,
                            operation_mode: rx_config.fifo0_operating_mode,
                            watermark_level: rx_config.fifo0_watermark_level,
                            size: rx_config.fifo0_size,
                            start_address: rx_config.rx_fifo0_start_address,
                        });
                    }
                    RxMode::Fifo1 => {
                        self.set_rx_fifo1(FifoData {
                            field_size: rx_config.fifo1_data_field_size,
                            operation_mode: rx_config.fifo1_operating_mode,
                            watermark_level: rx_config.fifo1_watermark_level,
                            size: rx_config.fifo1_size,
                            start_address: rx_config.rx_fifo1_start_address,
                        });
                    }
                }

                self.set_frame_mode(self.frame_mode);
            }

            // TODO I think this should accept pins as provided by gpio module
            pub fn setup_pins(&self, pins: &Pins<$ModuleId, I>) {
                self.connect_pin_rx(
                    &pins.rx,
                    InputMode::PULL_UP,
                    PadDriver::CmosAutomotiveSpeed3,
                );
                self.connect_pin_tx(
                    &pins.tx,
                    OutputMode::PUSH_PULL,
                    PadDriver::CmosAutomotiveSpeed3,
                );
            }

            pub fn setup_interrupt(&self, interrupt: &NodeInterruptConfig) {
                self.set_interrupt(
                    interrupt.interrupt_group,
                    interrupt.interrupt,
                    interrupt.line,
                    interrupt.priority,
                    interrupt.tos,
                );
            }

            fn set_rx_fifo0(&self, data: FifoData) {
                self.effects.set_rx_fifo0_data_field_size(data.field_size);
                self.effects.set_rx_fifo0_start_address(data.start_address);
                self.effects.set_rx_fifo0_size(data.size);
                self.effects
                    .set_rx_fifo0_operating_mode(data.operation_mode);
                self.effects
                    .set_rx_fifo0_watermark_level(data.watermark_level);
            }

            fn set_rx_fifo1(&self, data: FifoData) {
                self.effects.set_rx_fifo1_data_field_size(data.field_size);
                self.effects.set_rx_fifo1_start_address(data.start_address);
                self.effects.set_rx_fifo1_size(data.size);
                self.effects
                    .set_rx_fifo1_operating_mode(data.operation_mode);
                self.effects
                    .set_rx_fifo1_watermark_level(data.watermark_level);
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
                    if let Ok(tx_buffer_id) = TxBufferId::try_from(id) {
                        self.effects
                            .enable_tx_buffer_transmission_interrupt(tx_buffer_id);
                    }
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
            fn set_tx_buffer_data_field_size(&self, data_field_size: DataFieldSize) {
                self.effects
                    .set_tx_buffer_data_field_size(data_field_size.to_esci_register_value());
            }

            #[inline]
            fn set_rx_buffer_data_field_size(&self, data_field_size: DataFieldSize) {
                self.effects.set_rx_buffer_data_field_size(data_field_size);
            }

            fn set_frame_mode(&self, frame_mode: FrameMode) {
                let (fdoe, brse) = match frame_mode {
                    FrameMode::Standard => (false, false),
                    FrameMode::FdLong => (true, false),
                    FrameMode::FdLongAndFast => (true, true),
                };

                self.effects.set_frame_mode(fdoe, brse);
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

                <$ModuleId>::service_request(line).enable(priority, tos);

                // Enable interrupt
                self.effects.enable_interrupt(interrupt);
            }

            fn set_group_interrupt_line(&self, group: InterruptGroup, line: InterruptLine) {
                let line = u32::from(u8::from(line));
                let group = u32::from(u8::from(group));

                if group < 8 {
                    let group = group * 4;
                    self.effects.set_interrupt_routing_group_1(line, group);
                } else {
                    let group = (group % 8) * 4;
                    self.effects.set_interrupt_routing_group_2(line, group);
                }
            }

            fn connect_pin_rx(
                &self,
                rxd: &RxdIn<$ModuleId, I>,
                mode: InputMode,
                pad_driver: PadDriver,
            ) {
                let port = Port::new(rxd.port);
                port.set_pin_mode_input(rxd.pin_index, mode);
                port.set_pin_pad_driver(rxd.pin_index, pad_driver);
                self.effects.connect_pin_rx(rxd.select);
            }

            fn connect_pin_tx(
                &self,
                txd: &TxdOut<$ModuleId, I>,
                mode: OutputMode,
                pad_driver: PadDriver,
            ) {
                let port = Port::new(txd.port);
                port.set_pin_mode_output(txd.pin_index, mode, txd.select);
                port.set_pin_pad_driver(txd.pin_index, pad_driver);
                port.set_pin_low(txd.pin_index);
            }
        }

        // Methods only valid on a configured node
        impl<I: NodeId> Node<$NodeReg, $ModuleReg, I, Configured> {
            // TODO This does not feel to be the right place for this function
            pub fn clear_interrupt_flag(&self, interrupt: Interrupt) {
                self.effects.clear_interrupt_flag(interrupt);
            }

            pub fn transmit(&self, frame: &Frame) -> Result<(), TransmitError> {
                let buffer_id = self.get_tx_fifo_queue_put_index();
                self.transmit_inner(buffer_id, frame.id, false, false, false, frame.data)
            }

            pub fn receive(&self, from: ReadFrom, data: &mut [u8]) -> Option<RxMessage> {
                let Some(rx_config) = self.rx_config else {
                    return None;
                };

                let buffer_id = match from {
                    ReadFrom::RxFifo0 => self.effects.get_rx_fifo0_get_index(),
                    ReadFrom::RxFifo1 => self.effects.get_rx_fifo1_get_index(),
                    ReadFrom::Buffer(id) => id,
                };

                let rx_buf_elem = self.effects.get_rx_element_address(
                    self.ram_base_address,
                    match from {
                        ReadFrom::RxFifo0 => rx_config.rx_fifo0_start_address,
                        ReadFrom::RxFifo1 => rx_config.rx_fifo1_start_address,
                        ReadFrom::Buffer(_) => rx_config.rx_buffers_start_address,
                    },
                    from,
                    buffer_id,
                );

                // info!("read message on buffer_id: {}", buffer_id.0);
                // info!("rx_buf_elem at: {:x}", rx_buf_elem.get_ptr());

                let id = MessageId {
                    data: rx_buf_elem.get_message_id(),
                    length: rx_buf_elem.get_message_id_length(),
                };

                let data_length_code = rx_buf_elem.get_data_length();
                let frame_mode = rx_buf_elem.get_frame_mode();

                rx_buf_elem.read_data(data_length_code, data.as_mut_ptr());

                match from {
                    ReadFrom::RxFifo0 => self.effects.set_rx_fifo0_acknowledge_index(buffer_id),
                    ReadFrom::RxFifo1 => self.effects.set_rx_fifo1_acknowledge_index(buffer_id),
                    ReadFrom::Buffer(_) => (),
                }

                self.effects.clear_rx_buffer_new_data_flag(buffer_id);

                Some(RxMessage {
                    id,
                    data_length_code,
                    frame_mode,
                    buffer_id,
                    from,
                })
            }

            fn get_tx_fifo_queue_put_index(&self) -> TxBufferId {
                let id = self.effects.get_tx_fifo_queue_put_index() & 0x1F;
                // SAFETY The value is in range because it is read from a register and masked with 0x1F
                unsafe { TxBufferId::try_from(id).unwrap_unchecked() }
            }

            #[allow(unused_variables)]
            fn transmit_inner(
                &self,
                buffer_id: TxBufferId,
                id: MessageId,
                tx_event_fifo_control: bool,
                remote_transmit_request: bool,
                error_state_indicator: bool,
                data: &[u8],
            ) -> Result<(), TransmitError> {
                let req_pending = self.effects.is_tx_buffer_request_pending(buffer_id);
                if req_pending {
                    return Err(TransmitError::Busy);
                }

                let tx_buf_el = self.get_tx_element_address(self.ram_base_address, buffer_id);

                tx_buf_el.set_msg_id(id);

                if tx_event_fifo_control {
                    tx_buf_el.set_tx_event_fifo_ctrl(tx_event_fifo_control);
                    tx_buf_el.set_message_marker(buffer_id);
                }

                tx_buf_el.set_remote_transmit_req(remote_transmit_request);

                if let FrameMode::FdLong | FrameMode::FdLongAndFast = self.frame_mode {
                    tx_buf_el.set_err_state_indicator(error_state_indicator)
                }

                let dlc = DataLenghtCode::from_length(data.len())
                    .ok_or(TransmitError::InvalidDataLength)?;

                tx_buf_el.set_data_length(dlc);
                tx_buf_el.write_tx_buf_data(dlc, data.as_ptr());
                tx_buf_el.set_frame_mode_req(self.frame_mode);
                let buffer_id = 0; //TODO
                self.effects.set_tx_buffer_add_request(buffer_id);

                info!("transmit {}#{}", id.data, crate::log::HexSlice::from(data));

                Ok(())
            }

            fn get_tx_element_address(
                &self,
                ram_base_address: u32,
                buffer_number: TxBufferId,
            ) -> Tx {
                let num_of_config_bytes = 8u32;
                let num_of_data_bytes = self.effects.get_tx_buffer_data_field_size() as u32;
                let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
                let tx_buffer_index = tx_buffer_size * u32::from(u8::from(buffer_number));

                let tx_buffer_element_address =
                    ram_base_address + TX_BUFFER_START_ADDRESS + tx_buffer_index;

                Tx::new(tx_buffer_element_address as *mut u8)
            }

            // TODO Untested, example missing
            #[inline]
            fn is_tx_buffer_cancellation_finished(&self, tx_buffer_id: TxBufferId) -> bool {
                self.is_tx_buffer_transmission_occured(tx_buffer_id)
            }

            // TODO Untested, example missing
            #[inline]
            fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: TxBufferId) -> bool {
                self.effects
                    .is_tx_buffer_transmission_occured(tx_buffer_id.into())
            }
        }
    };
}

impl_can_node!(
    crate::pac::can0::Can0,
    crate::pac::can0::N,
    crate::can::Module0
);
impl_can_node!(
    crate::pac::can1::Can1,
    crate::pac::can1::N,
    crate::can::Module1
);

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
    fn to_esci_register_value(self) -> u8 {
        match self {
            DataFieldSize::_8 => 0,
            DataFieldSize::_12 => 1,
            DataFieldSize::_16 => 2,
            DataFieldSize::_20 => 3,
            DataFieldSize::_24 => 4,
            DataFieldSize::_32 => 5,
            DataFieldSize::_48 => 6,
            DataFieldSize::_64 => 7,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DedicatedData {
    pub field_size: DataFieldSize,
    pub start_address: u16,
}

#[repr(u8)]
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

impl From<InterruptGroup> for u8 {
    fn from(value: InterruptGroup) -> Self {
        value as u8
    }
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
    // TODO: reserved
    BitErrorCorrected,
    // TODO: reserved
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

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Default)]
pub enum InterruptLine {
    #[default]
    Line0,
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8,
    Line9,
    Line10,
    Line11,
    Line12,
    Line13,
    Line14,
    Line15,
}

impl From<InterruptLine> for u8 {
    fn from(value: InterruptLine) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Tos {
    #[default]
    Cpu0,
    Dma,
    Cpu1,
    Cpu2,
}

impl From<Tos> for u8 {
    fn from(value: Tos) -> Self {
        match value {
            Tos::Cpu0 => 0,
            Tos::Dma => 1,
            Tos::Cpu1 => 2,
            Tos::Cpu2 => 3,
        }
    }
}

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

struct Port {
    inner: tc37x::p00::P00,
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

// TODO Is this needed? Can we get rid of it? Seems to be a duplicate of gpio
impl Port {
    fn new(port: PortNumber) -> Self {
        use tc37x::p00::P00;
        use tc37x::*;

        let inner: P00 = match port {
            PortNumber::_00 => P00,
            PortNumber::_01 => unsafe { transmute(P01) },
            PortNumber::_02 => unsafe { transmute(P02) },
            PortNumber::_10 => unsafe { transmute(P10) },
            PortNumber::_11 => unsafe { transmute(P11) },
            PortNumber::_12 => unsafe { transmute(P12) },
            PortNumber::_13 => unsafe { transmute(P13) },
            PortNumber::_14 => unsafe { transmute(P14) },
            PortNumber::_15 => unsafe { transmute(P15) },
            PortNumber::_20 => unsafe { transmute(P20) },
            PortNumber::_21 => unsafe { transmute(P21) },
            PortNumber::_22 => unsafe { transmute(P22) },
            PortNumber::_23 => unsafe { transmute(P23) },
            PortNumber::_32 => unsafe { transmute(P32) },
            PortNumber::_33 => unsafe { transmute(P33) },
            PortNumber::_34 => unsafe { transmute(P34) },
            PortNumber::_40 => unsafe { transmute(P40) },
        };
        Self { inner }
    }

    fn set_pin_state(&self, index: u8, action: State) {
        unsafe {
            self.inner.omr().init(|r| {
                let v = (action as u32) << index;
                r.set_raw(v)
            })
        };
    }

    fn toogle_pin(&self, index: u8) {
        self.set_pin_state(index, State::Toggled)
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

        // TODO This unsafe code could be made safe by comparing the address (usize) of the port if only self.inner.0 was public
        let is_supervisor =
            unsafe { transmute::<_, usize>(self.inner) } == unsafe { transmute(crate::pac::P40) };

        if is_supervisor {
            wdt_call::call_without_cpu_endinit(|| unsafe {
                self.inner.pdisc().modify(|r| {
                    // TODO Check if the new version is compatible with the previous one:
                    // *r.data_mut_ref() &= !(1 << index);

                    let mut v = r.get_raw();
                    v &= !(1 << index);
                    r.set_raw(v)
                })
            });
        }

        // TODO Can we do this without transmute?
        // TODO Use change_pin_mode_port_pin from gpio module instead?
        let iocr: crate::pac::Reg<crate::pac::p00::Iocr0_SPEC, crate::pac::RW> = {
            let iocr0 = self.inner.iocr0();
            let addr: *mut u32 = unsafe { transmute(iocr0) };
            let addr = unsafe { addr.add(ioc_index as usize) };
            unsafe { transmute(addr) }
        };

        let v : u32 = (mode.0) << shift;
        let m : u32 = 0xFFu32 << shift;

        unsafe {
            core::arch::tricore::intrinsics::__ldmst(iocr.ptr(), v, m);
        }
    }

    fn set_pin_pad_driver(&self, index: u8, driver: PadDriver) {
        let pdr_index = index / 8;
        let shift = (index & 0x7) * 4;
        let pdr: crate::pac::Reg<crate::pac::p00::Pdr0_SPEC, crate::pac::RW> = {
            let pdr0 = self.inner.pdr0();
            let addr: *mut u32 = unsafe { transmute(pdr0) };
            let addr = unsafe { addr.add(pdr_index as usize) };
            unsafe { transmute(addr) }
        };

        wdt_call::call_without_cpu_endinit(|| unsafe {
            let v : u32 = (driver as u32) << shift;
            let m : u32 = 0xF << shift;
            unsafe {
                core::arch::tricore::intrinsics::__ldmst(pdr.ptr(), v, m);
            }
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
pub struct OutputIdx(u32);
impl OutputIdx {
    pub(crate) const GENERAL: Self = Self(0x10 << 3);
    pub(crate) const ALT1: Self = Self(0x11 << 3);
    pub(crate) const ALT2: Self = Self(0x12 << 3);
    pub(crate) const ALT3: Self = Self(0x13 << 3);
    pub(crate) const ALT4: Self = Self(0x14 << 3);
    pub(crate) const ALT5: Self = Self(0x15 << 3);
    pub(crate) const ALT6: Self = Self(0x16 << 3);
    pub(crate) const ALT7: Self = Self(0x17 << 3);
}

#[derive(Clone, Copy)]
pub struct RxdIn<M, N> {
    port: PortNumber,
    pin_index: u8,
    select: RxSel,
    _phantom: PhantomData<(M, N)>,
}

impl<M, N> RxdIn<M, N> {
    pub(crate) const fn new(port: PortNumber, pin_index: u8, select: RxSel) -> Self {
        Self {
            port,
            pin_index,
            select,
            _phantom: PhantomData,
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

impl From<RxSel> for u8 {
    fn from(value: RxSel) -> Self {
        match value {
            RxSel::_A => 0,
            RxSel::_B => 1,
            RxSel::_C => 2,
            RxSel::_D => 3,
            RxSel::_E => 4,
            RxSel::_F => 5,
            RxSel::_G => 6,
            RxSel::_H => 7,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TxdOut<M: ModuleId, N: NodeId> {
    port: PortNumber,
    pin_index: u8,
    select: OutputIdx,
    _phantom: PhantomData<(M, N)>,
}

impl<M: ModuleId, N: NodeId> TxdOut<M, N> {
    pub(crate) const fn new(port: PortNumber, pin_index: u8, select: OutputIdx) -> Self {
        Self {
            port,
            pin_index,
            select,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TxConfig {
    pub mode: TxMode,
    pub dedicated_tx_buffers_number: u8,
    pub fifo_queue_size: u8,
    pub buffer_data_field_size: DataFieldSize,
    pub event_fifo_size: u8,
    pub tx_event_fifo_start_address: u16,
    pub tx_buffers_start_address: u16,
}

//     pub standard_filter_list_start_address: u16,
//     pub extended_filter_list_start_address: u16,

#[derive(Clone, Copy)]
pub struct RxConfig {
    pub mode: RxMode,
    pub buffer_data_field_size: DataFieldSize,
    pub fifo0_data_field_size: DataFieldSize,
    pub fifo1_data_field_size: DataFieldSize,
    pub fifo0_operating_mode: RxFifoMode,
    pub fifo1_operating_mode: RxFifoMode,
    pub fifo0_watermark_level: u8,
    pub fifo1_watermark_level: u8,
    pub fifo0_size: u8,
    pub fifo1_size: u8,
    pub rx_fifo0_start_address: u16,
    pub rx_fifo1_start_address: u16,
    pub rx_buffers_start_address: u16,
}

#[derive(Clone, Copy)]
pub struct Pins<M: ModuleId, N: NodeId> {
    pub tx: TxdOut<M, N>,
    pub rx: RxdIn<M, N>,
}
