// TODO Remove this once the module is more complete
#![allow(dead_code)]

mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod internals;
mod msg;
mod reg;

pub use baud_rate::{AutoBitTiming, BitTiming, BitTimingConfig};
pub use can_module::{Module, ModuleConfig, ModuleId, can_module0, can_module1};
pub use can_node::{Node, NodeConfig, DataFieldSize, NodeId, TxConfig, TxMode};
pub use frame::Frame;