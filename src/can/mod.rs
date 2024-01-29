// TODO Remove this once the module is more complete
#![allow(dead_code)]

#![deny(clippy::result_unit_err)]

mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod internals;
pub mod msg;
mod reg;

pub use baud_rate::{AutoBitTiming, BitTimingConfig, DataBitTiming, NominalBitTiming};
pub use can_module::{Module, ModuleConfig, ModuleId};
pub use can_node::{DataFieldSize, Node, NodeConfig, NodeId, TxConfig, TxMode, RxConfig};
pub use can_node::TXD00_P20_8_OUT;
pub use can_node::RXD00B_P20_7_IN;
pub use frame::Frame;
pub use msg::MessageId;
