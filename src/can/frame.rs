#![allow(unused_variables)]

use embedded_can::Id;

pub struct Frame;

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(Self)
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        todo!()
    }

    fn is_extended(&self) -> bool {
        todo!()
    }

    fn is_remote_frame(&self) -> bool {
        todo!()
    }

    fn id(&self) -> Id {
        todo!()
    }

    fn dlc(&self) -> usize {
        todo!()
    }

    fn data(&self) -> &[u8] {
        todo!()
    }
}
