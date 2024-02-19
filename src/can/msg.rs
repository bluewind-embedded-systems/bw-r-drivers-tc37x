use super::frame::DataLenghtCode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageIdLenght {
    Standard,
    Extended,
    Both,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MessageId {
    pub data: u32,
    pub length: MessageIdLenght,
}

impl From<embedded_can::Id> for MessageId {
    fn from(id: embedded_can::Id) -> Self {
        match id {
            embedded_can::Id::Standard(id) => id.into(),
            embedded_can::Id::Extended(id) => id.into(),
        }
    }
}

impl From<embedded_can::StandardId> for MessageId {
    fn from(id: embedded_can::StandardId) -> Self {
        MessageId {
            data: id.as_raw().into(),
            length: MessageIdLenght::Standard,
        }
    }
}

impl From<embedded_can::ExtendedId> for MessageId {
    fn from(id: embedded_can::ExtendedId) -> Self {
        MessageId {
            data: id.as_raw(),
            length: MessageIdLenght::Extended,
        }
    }
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum FrameMode {
    #[default]
    Standard,
    FdLong,
    FdLongAndFast,
}

#[derive(Debug)]
pub struct RxMessage {
    pub buffer_id: RxBufferId,
    pub id: MessageId,
    pub data_length_code: DataLenghtCode,
    pub from: ReadFrom,
    pub frame_mode: FrameMode,
}

#[derive(Debug)]
pub struct TxMessage {
    pub id: MessageId,
    pub buffer_id: Option<TxBufferId>,
    pub remote_transmit_request: bool,
    pub error_state_indicator: bool,
    pub tx_event_fifo_control: bool,
    pub data_length_code: DataLenghtCode,
    pub frame_mode: FrameMode,
    pub from: ReadFrom,
}

impl Default for TxMessage {
    fn default() -> Self {
        Self {
            id: MessageId {
                data: 0,
                length: MessageIdLenght::Standard,
            },
            buffer_id: Some(TxBufferId(0)),
            remote_transmit_request: false,
            error_state_indicator: false,
            tx_event_fifo_control: false,
            data_length_code: DataLenghtCode::_8,
            frame_mode: FrameMode::Standard,
            from: ReadFrom::Buffer(RxBufferId(0)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FrameType {
    Receive,
    Transmit,
    TransmitAndReceive,
    RemoteRequest,
    RemoteAnswer,
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TxBufferId(u8);

impl TryFrom<u8> for TxBufferId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX: u8 = 31;
        if value > MAX {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl From<TxBufferId> for u8 {
    fn from(value: TxBufferId) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct RxBufferId(u8);

impl RxBufferId {
    pub const MAX: u8 = 63;

    pub fn new(n: u8) -> Option<Self> {
        match n {
            ..=Self::MAX => Some(Self(n)),
            _ => None,
        }
    }

    pub unsafe fn new_unchecked(n: u8) -> Self {
        debug_assert!(n <= Self::MAX);
        Self(n)
    }
}

impl From<RxBufferId> for u8 {
    fn from(value: RxBufferId) -> Self {
        value.0
    }
}

impl From<RxBufferId> for u16 {
    fn from(value: RxBufferId) -> Self {
        value.0.into()
    }
}

impl From<RxBufferId> for u32 {
    fn from(value: RxBufferId) -> Self {
        value.0.into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ReadFrom {
    RxFifo0,
    RxFifo1,
    Buffer(RxBufferId),
}

#[derive(Clone, Copy, PartialEq)]
pub enum FilterElementConfiguration {
    Disable,
    StoreInRxFifo0,
    StoreInRxFifo1,
    RejectId,
    SetPriority,
    SetPriorityAndStoreInFifo0,
    SetPriorityAndStoreInFifo1,
    StoreInRxBuffer,
}

#[derive(Clone, Copy)]
pub enum FilterType {
    Range,
    Dualid,
    Classic,
    None,
}

#[derive(Clone, Copy)]
pub struct Filter {
    pub number: u8,
    pub element_configuration: FilterElementConfiguration,
    pub typ: FilterType,
    pub id1: u32,
    pub id2: u32,
    pub rx_buffer_offset: RxBufferId,
}
