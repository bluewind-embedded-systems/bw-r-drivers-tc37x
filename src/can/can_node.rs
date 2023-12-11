use super::frame::Frame;
use super::CanModule;

#[derive(Default)]
pub struct CanNodeConfig {}

pub struct NodeId(u8);

impl NodeId {
    pub const fn new(n: u8) -> Self {
        Self(n)
    }
}

pub struct CanNode {
    module: CanModule,
    node_id: NodeId,
}

impl CanNode {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: CanModule, node_id: NodeId) -> Self {
        Self { module, node_id }
    }

    pub fn init(self, _config: CanNodeConfig) -> Result<CanNode, ()> {
        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}
