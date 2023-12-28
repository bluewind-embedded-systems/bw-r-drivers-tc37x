#![allow(unused_variables)]

use embedded_can::{Id, StandardId};

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
