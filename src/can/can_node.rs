use super::can_module::ClockSource;
use super::frame::Frame;
use super::CanModule;
use crate::util::wait_nop;

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
}

pub struct NodeId(pub(crate) u8);

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

    pub fn init(self, config: CanNodeConfig) -> Result<CanNode, ()> {
        let node_id = self.node_id;

        self.module
            .set_clock_source(node_id.into(), config.clock_source);

        wait_nop(10);

        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}
