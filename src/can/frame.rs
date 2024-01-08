#![allow(unused_variables)]

use embedded_can::{Id, StandardId};

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

impl DataLenghtCode {
    pub fn get_data_lenght_in_bytes(&self) -> u32 {
        match *self {
            n if n <= Self::_8 => n as u32,
            n if n <= Self::_24 => (n as u32 - 6) << 2,
            n => (n as u32 - 11) << 4,
        }
    }

    pub fn get_data_lenght_int32(&self) -> u32 {
        let num_byts = match *self {
            n if n <= Self::_8 => n as u32,
            n if n <= Self::_24 => (n as u32 - 6) << 2,
            n => (n as u32 - 11) << 4,
        };

        (num_byts + 3) >> 2
    }
}

pub struct Frame;

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(Self)
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        todo!()
    }

    fn is_extended(&self) -> bool {
        false
        //TODO
    }

    fn is_remote_frame(&self) -> bool {
        false
        //TODO
    }

    fn id(&self) -> Id {
        Id::Standard(StandardId::new(123).unwrap())
        //TODO
    }

    fn dlc(&self) -> usize {
        8
        //TODO
    }

    fn data(&self) -> &[u8] {
        &[1, 2, 3, 4, 5, 6, 7, 8]
    }
}

#[cfg(test)]
mod test {
    use super::DataLenghtCode;

    #[test]
    pub fn test_data_lenght_code() {
        let test = |code: DataLenghtCode| {
            assert_eq!(
                code.get_data_lenght_in_bytes(),
                match code {
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
        for i in 0u8..=DataLenghtCode::_64 as _ {
            test(DataLenghtCode::try_from(i).unwrap());
        }
    }

    #[test]
    pub fn test_data_lenght_int32() {
        let test = |code: DataLenghtCode| {
            assert_eq!(
                code.get_data_lenght_int32(),
                match code {
                    DataLenghtCode::_0 => 0,
                    DataLenghtCode::_1 => 1,
                    DataLenghtCode::_2 => 1,
                    DataLenghtCode::_3 => 1,
                    DataLenghtCode::_4 => 1,
                    DataLenghtCode::_5 => 2,
                    DataLenghtCode::_6 => 2,
                    DataLenghtCode::_7 => 2,
                    DataLenghtCode::_8 => 2,
                    DataLenghtCode::_12 => 3,
                    DataLenghtCode::_16 => 4,
                    DataLenghtCode::_20 => 5,
                    DataLenghtCode::_24 => 6,
                    DataLenghtCode::_32 => 8,
                    DataLenghtCode::_48 => 12,
                    DataLenghtCode::_64 => 16,
                }
            );
        };
        for i in 0u8..=DataLenghtCode::_64 as _ {
            test(DataLenghtCode::try_from(i).unwrap());
        }
    }
}
