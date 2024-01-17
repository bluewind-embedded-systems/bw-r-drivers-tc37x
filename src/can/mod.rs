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
pub use can_module::{Module, ModuleConfig, ModuleId};
pub use can_node::{Node, NodeConfig, DataFieldSize, NodeId, TxConfig, TxMode};
pub use frame::Frame;
pub use msg::TxBufferId;
