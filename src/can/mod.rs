mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod reg; 
mod internals;
mod msg; 

pub use can_module::{CanModule, CanModuleConfig, CanModuleId};
pub use can_node::{CanNode, CanNodeConfig, NewCanNode, NodeId};
pub use msg::{TxBufferId};
pub use frame::Frame;
