use super::frame::DataLenghtCode;

// #[repr(transparent)]
// #[derive(Clone, Copy)]
// pub struct NodeId(pub u8);

// impl NodeId {
//     const MAX: u8 = 3;

//     pub fn new(n: u8) -> Option<Self> {
//         match n {
//             ..=Self::MAX => Some(Self(n)),
//             _ => None,
//         }
//     }

//     pub const fn new_const(n: u8) -> Self {
//         match n {
//             ..=Self::MAX => Self(n),
//             _ => panic!("over the max range"),
//         }
//     }
// }

// impl From<NodeId> for usize {
//     fn from(value: NodeId) -> Self {
//         value.0.into()
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub enum ReadFrom {
//     RxFifo0,
//     RxFifo1,
//     Buffer(RxBufferId),
// }

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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FrameMode {
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
pub struct TxBufferId(pub u8);

impl TxBufferId {
    pub const MAX: u8 = 31;

    pub fn new(n: u8) -> Option<Self> {
        match n {
            ..=Self::MAX => Some(Self(n)),
            _ => None,
        }
    }

    pub const fn new_const(n: u8) -> Self {
        match n {
            ..=Self::MAX => Self(n),
            _ => panic!("over the max range"),
        }
    }
}

impl From<TxBufferId> for u8 {
    fn from(value: TxBufferId) -> Self {
        value.0
    }
}

impl From<TxBufferId> for u16 {
    fn from(value: TxBufferId) -> Self {
        value.0.into()
    }
}

impl From<TxBufferId> for u32 {
    fn from(value: TxBufferId) -> Self {
        value.0.into()
    }
}

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct RxBufferId(pub u8);

impl RxBufferId {
    pub const MAX: u8 = 63;

    pub fn new(n: u8) -> Option<Self> {
        match n {
            ..=Self::MAX => Some(Self(n)),
            _ => None,
        }
    }

    pub const fn new_const(n: u8) -> Self {
        match n {
            ..=Self::MAX => Self(n),
            _ => panic!("over the max range"),
        }
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
// #[repr(u8)]
// #[derive(Clone, Copy)]
// pub enum DataFieldSize {
//     _8,
//     _12,
//     _16,
//     _20,
//     _24,
//     _32,
//     _48,
//     _64,
// }

// impl From<DataFieldSize> for u8 {
//     fn from(value: DataFieldSize) -> Self {
//         value as _
//     }
// }

// impl From<DataFieldSize> for pac::can0::node::txesc::Tbds {
//     fn from(value: DataFieldSize) -> Self {
//         u64::from(value as u8).cast_to()
//     }
// }

// impl TryFrom<u8> for DataFieldSize {
//     type Error = ();

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         Ok(match value {
//             0 => Self::_8,
//             1 => Self::_12,
//             2 => Self::_16,
//             3 => Self::_20,
//             4 => Self::_24,
//             5 => Self::_32,
//             6 => Self::_48,
//             7 => Self::_64,
//             _ => return Err(()),
//         })
//     }
// }

// impl From<DataFieldSize> for pac::can0::node::rxesc::Rbds {
//     fn from(value: DataFieldSize) -> Self {
//         Self(value.into())
//     }
// }

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
