#![allow(unused_variables)]

use crate::can::msg::MessageId;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum DataLenghtCode {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _12,
    _16,
    _20,
    _24,
    _32,
    _48,
    _64,
}

impl DataLenghtCode {
    pub const fn from_length(length: usize) -> Option<Self> {
        match length {
            0 => Some(Self::_0),
            1 => Some(Self::_1),
            2 => Some(Self::_2),
            3 => Some(Self::_3),
            4 => Some(Self::_4),
            5 => Some(Self::_5),
            6 => Some(Self::_6),
            7 => Some(Self::_7),
            8 => Some(Self::_8),
            12 => Some(Self::_12),
            16 => Some(Self::_16),
            20 => Some(Self::_20),
            24 => Some(Self::_24),
            32 => Some(Self::_32),
            48 => Some(Self::_48),
            64 => Some(Self::_64),
            _ => None,
        }
    }

    pub const fn to_length(self) -> usize {
        match self {
            Self::_0 => 0,
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_3 => 3,
            Self::_4 => 4,
            Self::_5 => 5,
            Self::_6 => 6,
            Self::_7 => 7,
            Self::_8 => 8,
            Self::_12 => 12,
            Self::_16 => 16,
            Self::_20 => 20,
            Self::_24 => 24,
            Self::_32 => 32,
            Self::_48 => 48,
            Self::_64 => 64,
        }
    }
}

impl TryFrom<u8> for DataLenghtCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::_0,
            1 => Self::_1,
            2 => Self::_2,
            3 => Self::_3,
            4 => Self::_4,
            5 => Self::_5,
            6 => Self::_6,
            7 => Self::_7,
            8 => Self::_8,
            9 => Self::_12,
            10 => Self::_16,
            11 => Self::_20,
            12 => Self::_24,
            13 => Self::_32,
            14 => Self::_48,
            15 => Self::_64,
            _ => return Err(()),
        })
    }
}

impl From<DataLenghtCode> for u8 {
    fn from(value: DataLenghtCode) -> Self {
        value as u8
    }
}

pub struct Frame<'a> {
    pub id: MessageId,
    pub data: &'a [u8],
}

impl<'a> Frame<'a> {
    pub fn new(id: MessageId, data: &'a [u8]) -> Option<Self> {
        if data.len() > 64 {
            None
        } else {
            Some(Self { id, data })
        }
    }
}

#[cfg(test)]
mod test {
    use super::DataLenghtCode;

    #[test]
    pub fn test_data_length_code() {
        let test = |dlc: DataLenghtCode| {
            assert_eq!(
                dlc.to_length(),
                match dlc {
                    DataLenghtCode::_0 => 0,
                    DataLenghtCode::_1 => 1,
                    DataLenghtCode::_2 => 2,
                    DataLenghtCode::_3 => 3,
                    DataLenghtCode::_4 => 4,
                    DataLenghtCode::_5 => 5,
                    DataLenghtCode::_6 => 6,
                    DataLenghtCode::_7 => 7,
                    DataLenghtCode::_8 => 8,
                    DataLenghtCode::_12 => 12,
                    DataLenghtCode::_16 => 16,
                    DataLenghtCode::_20 => 20,
                    DataLenghtCode::_24 => 24,
                    DataLenghtCode::_32 => 32,
                    DataLenghtCode::_48 => 48,
                    DataLenghtCode::_64 => 64,
                }
            );
        };
        for i in 0u8..=DataLenghtCode::_64 as u8 {
            test(DataLenghtCode::try_from(i).unwrap());
        }
    }
}
