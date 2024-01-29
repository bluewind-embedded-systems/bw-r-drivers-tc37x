// TODO Remove this once the module is more complete
#![allow(dead_code)]
#![deny(clippy::result_unit_err)]

mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod internals;
pub mod msg;
pub mod pin_map;
mod reg;

pub use baud_rate::{AutoBitTiming, BitTimingConfig, DataBitTiming, NominalBitTiming};
pub use can_module::{Module, ModuleConfig, ModuleId};
pub use can_node::{DataFieldSize, Node, NodeConfig, NodeId, RxConfig, TxConfig, TxMode};
pub use frame::Frame;
pub use msg::MessageId;
