// TODO Remove this once the module is more complete
#![allow(dead_code)]

mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod internals;
mod msg;
mod reg;

pub use can_module::{CanModule, CanModuleConfig, CanModuleId};
pub use can_node::{
    CanNode, CanNodeConfig, DataFieldSize, NewCanNode, NodeId, TxConfig, TxMode,
};
pub use frame::Frame;
pub use msg::TxBufferId;
