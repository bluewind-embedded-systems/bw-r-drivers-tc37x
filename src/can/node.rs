
pub trait ACanNode{

}

pub struct Node<const N: u8, const M:u8>; 

// impl ACanNode for Node<0, 0>{

// }

impl<const N: u8, const M:u8> ACanNode for Node<N,  M>{

}
// use tc37x_pac::{self as pac, CastTo};

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

// #[repr(u8)]
// #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
// pub enum DataLenghtCode {
//     _0,
//     _1,
//     _2,
//     _3,
//     _4,
//     _5,
//     _6,
//     _7,
//     _8,
//     _12,
//     _16,
//     _20,
//     _24,
//     _32,
//     _48,
//     _64,
// }

// impl TryFrom<u8> for DataLenghtCode {
//     type Error = ();

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         Ok(match value {
//             0 => Self::_0,
//             1 => Self::_1,
//             2 => Self::_2,
//             3 => Self::_3,
//             4 => Self::_4,
//             5 => Self::_5,
//             6 => Self::_6,
//             7 => Self::_7,
//             8 => Self::_8,
//             9 => Self::_12,
//             10 => Self::_16,
//             11 => Self::_20,
//             12 => Self::_24,
//             13 => Self::_32,
//             14 => Self::_48,
//             15 => Self::_64,
//             _ => return Err(()),
//         })
//     }
// }

// impl DataLenghtCode {
//     pub fn get_data_lenght_in_bytes(&self) -> u32 {
//         match *self {
//             n if n <= Self::_8 => n as u32,
//             n if n <= Self::_24 => (n as u32 - 6) << 2,
//             n => (n as u32 - 11) << 4,
//         }
//     }

//     pub fn get_data_lenght_int32(&self) -> u32 {
//         let num_byts = match *self {
//             n if n <= Self::_8 => n as u32,
//             n if n <= Self::_24 => (n as u32 - 6) << 2,
//             n => (n as u32 - 11) << 4,
//         };

//         (num_byts + 3) >> 2
//     }
// }

// impl From<DataFieldSize> for DataLenghtCode {
//     fn from(value: DataFieldSize) -> Self {
//         match value {
//             DataFieldSize::_8 => DataLenghtCode::_8,
//             DataFieldSize::_12 => DataLenghtCode::_12,
//             DataFieldSize::_16 => DataLenghtCode::_16,
//             DataFieldSize::_20 => DataLenghtCode::_20,
//             DataFieldSize::_24 => DataLenghtCode::_24,
//             DataFieldSize::_32 => DataLenghtCode::_32,
//             DataFieldSize::_48 => DataLenghtCode::_48,
//             DataFieldSize::_64 => DataLenghtCode::_64,
//         }
//     }
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum MessageIdLenght {
//     Standard,
//     Extended,
//     Both,
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MessageId {
//     pub data: u32,
//     pub lenght: MessageIdLenght,
// }

// #[derive(PartialEq, Clone, Copy, Debug)]
// pub enum FrameMode {
//     Standard,
//     FdLong,
//     FdLongAndFast,
// }

// #[derive(Debug)]
// pub struct RxMessage {
//     pub buffer_id: RxBufferId,
//     pub id: MessageId,
//     pub data_lenght_code: DataLenghtCode,
//     pub from: ReadFrom,
//     pub frame_mode: FrameMode,
// }

// #[derive(Debug)]
// pub struct TxMessage {
//     pub id: MessageId,
//     pub buffer_id: Option<TxBufferId>,
//     pub remote_transmit_request: bool,
//     pub error_state_indicator: bool,
//     pub tx_event_fifo_control: bool,
//     pub data_lenght_code: DataLenghtCode,
//     pub frame_mode: FrameMode,
//     pub from: ReadFrom,
// }

// impl Default for TxMessage {
//     fn default() -> Self {
//         Self {
//             id: MessageId {
//                 data: 0,
//                 lenght: MessageIdLenght::Standard,
//             },
//             buffer_id: Some(TxBufferId(0)),
//             remote_transmit_request: false,
//             error_state_indicator: false,
//             tx_event_fifo_control: false,
//             data_lenght_code: DataLenghtCode::_8,
//             frame_mode: FrameMode::Standard,
//             from: ReadFrom::Buffer(RxBufferId(0)),
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub enum FrameType {
//     Receive,
//     Transmit,
//     TransmitAndReceive,
//     RemoteRequest,
//     RemoteAnswer,
// }

// #[repr(u8)]
// #[derive(Clone, Copy)]
// pub enum ClockSelect {
//     _0,
//     _1,
//     _2,
//     _3,
// }

// impl From<ClockSelect> for u8 {
//     fn from(value: ClockSelect) -> Self {
//         value as _
//     }
// }

// impl From<NodeId> for ClockSelect {
//     fn from(value: NodeId) -> Self {
//         match usize::from(value) {
//             0 => Self::_0,
//             1 => Self::_1,
//             2 => Self::_2,
//             3 => Self::_3,
//             4.. => unimplemented!(),
//             _ => unimplemented!(),
//         }
//     }
// }

// #[repr(u8)]
// #[derive(Clone, Copy)]
// pub enum ClockSource {
//     NoClock,
//     Asynchronous,
//     Synchronous,
//     Both,
// }

// impl From<ClockSource> for u8 {
//     fn from(value: ClockSource) -> Self {
//         value as _
//     }
// }

// impl From<ClockSource> for pac::can0::mcr::Clksel {
//     fn from(value: ClockSource) -> Self {
//         Self(value as _)
//     }
// }

// #[derive(Clone, Copy, Debug)]
// #[repr(transparent)]
// pub struct TxBufferId(pub u8);

// impl TxBufferId {
//     pub const MAX: u8 = 31;

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

// impl From<TxBufferId> for u8 {
//     fn from(value: TxBufferId) -> Self {
//         value.0
//     }
// }

// impl From<TxBufferId> for u16 {
//     fn from(value: TxBufferId) -> Self {
//         value.0.into()
//     }
// }

// impl From<TxBufferId> for u32 {
//     fn from(value: TxBufferId) -> Self {
//         value.0.into()
//     }
// }

// #[repr(transparent)]
// #[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
// pub struct RxBufferId(pub u8);

// impl RxBufferId {
//     pub const MAX: u8 = 63;

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

// impl From<RxBufferId> for u8 {
//     fn from(value: RxBufferId) -> Self {
//         value.0
//     }
// }

// impl From<RxBufferId> for u16 {
//     fn from(value: RxBufferId) -> Self {
//         value.0.into()
//     }
// }

// impl From<RxBufferId> for u32 {
//     fn from(value: RxBufferId) -> Self {
//         value.0.into()
//     }
// }

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

// #[derive(Clone, Copy, PartialEq)]
// pub enum FilterElementConfiguration {
//     Disable,
//     StoreInRxFifo0,
//     StoreInRxFifo1,
//     RejectId,
//     SetPriority,
//     SetPriorityAndStoreInFifo0,
//     SetPriorityAndStoreInFifo1,
//     StoreInRxBuffer,
// }

// #[derive(Clone, Copy)]
// pub enum FilterType {
//     Range,
//     Dualid,
//     Classic,
//     None,
// }

// #[derive(Clone, Copy)]
// pub struct Filter {
//     pub number: u8,
//     pub element_configuration: FilterElementConfiguration,
//     pub typ: FilterType,
//     pub id1: u32,
//     pub id2: u32,
//     pub rx_buffer_offset: RxBufferId,
// }

// #[cfg(test)]
// mod test {
//     use super::DataLenghtCode;

//     #[test]
//     pub fn test_data_lenght_code() {
//         let test = |code: DataLenghtCode| {
//             assert_eq!(
//                 code.get_data_lenght_in_bytes(),
//                 match code {
//                     DataLenghtCode::_0 => 0,
//                     DataLenghtCode::_1 => 1,
//                     DataLenghtCode::_2 => 2,
//                     DataLenghtCode::_3 => 3,
//                     DataLenghtCode::_4 => 4,
//                     DataLenghtCode::_5 => 5,
//                     DataLenghtCode::_6 => 6,
//                     DataLenghtCode::_7 => 7,
//                     DataLenghtCode::_8 => 8,
//                     DataLenghtCode::_12 => 12,
//                     DataLenghtCode::_16 => 16,
//                     DataLenghtCode::_20 => 20,
//                     DataLenghtCode::_24 => 24,
//                     DataLenghtCode::_32 => 32,
//                     DataLenghtCode::_48 => 48,
//                     DataLenghtCode::_64 => 64,
//                 }
//             );
//         };
//         for i in 0u8..=DataLenghtCode::_64 as _ {
//             test(DataLenghtCode::try_from(i).unwrap());
//         }
//     }

//     #[test]
//     pub fn test_data_lenght_int32() {
//         let test = |code: DataLenghtCode| {
//             assert_eq!(
//                 code.get_data_lenght_int32(),
//                 match code {
//                     DataLenghtCode::_0 => 0,
//                     DataLenghtCode::_1 => 1,
//                     DataLenghtCode::_2 => 1,
//                     DataLenghtCode::_3 => 1,
//                     DataLenghtCode::_4 => 1,
//                     DataLenghtCode::_5 => 2,
//                     DataLenghtCode::_6 => 2,
//                     DataLenghtCode::_7 => 2,
//                     DataLenghtCode::_8 => 2,
//                     DataLenghtCode::_12 => 3,
//                     DataLenghtCode::_16 => 4,
//                     DataLenghtCode::_20 => 5,
//                     DataLenghtCode::_24 => 6,
//                     DataLenghtCode::_32 => 8,
//                     DataLenghtCode::_48 => 12,
//                     DataLenghtCode::_64 => 16,
//                 }
//             );
//         };
//         for i in 0u8..=DataLenghtCode::_64 as _ {
//             test(DataLenghtCode::try_from(i).unwrap());
//         }
//     }
// }
