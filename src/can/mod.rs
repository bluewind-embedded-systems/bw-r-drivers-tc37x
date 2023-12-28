mod baud_rate;
mod can_module;
mod can_node;
mod frame;
mod reg; 
pub use can_module::{CanModule, CanModuleConfig, CanModuleId};
pub use can_node::{CanNode, CanNodeConfig, NewCanNode, NodeId};
pub use frame::Frame;
