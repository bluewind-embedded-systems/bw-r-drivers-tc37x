// TODO Remove this once the module is more complete
#![allow(dead_code)]

mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod internals;
pub mod msg;
mod reg;

pub use baud_rate::{AutoBitTiming, BitTimingConfig, DataBitTiming, NominalBitTiming};
pub use can_module::{Module, ModuleConfig, ModuleId};
pub use can_node::{DataFieldSize, Node, NodeConfig, NodeId, TxConfig, TxMode};
pub use frame::Frame;
pub use msg::MessageId;
