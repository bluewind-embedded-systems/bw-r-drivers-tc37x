use super::can_module::ClockSource;
use super::frame::Frame;
use super::CanModule;
use crate::util::wait_nop;

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId(pub(crate) u8);

impl NodeId {
    pub const fn new(n: u8) -> Self {
        Self(n)
    }
}

pub struct CanNode {
    module: CanModule,
    node_id: NodeId,
    inner: tc37x_pac::can0::Node,
}

impl CanNode {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: CanModule, node_id: NodeId) -> Self {
        let inner = module.registers().node(node_id.0.into());
        Self {
            module,
            node_id,
            inner,
        }
    }

    pub fn init(self, config: CanNodeConfig) -> Result<CanNode, ()> {
        self.module
            .set_clock_source(self.node_id.into(), config.clock_source);

        wait_nop(10);

        self.enable_configuration_change();

        let module_freq = crate::scu::ccu::get_mcan_frequency();

        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }

    #[inline]
    fn enable_configuration_change(&self) {
        let cccr = self.inner.cccr();

        if unsafe { cccr.read() }.init().get() {
            unsafe { cccr.modify(|r| r.cce().set(false)) };
            while unsafe { cccr.read() }.cce().get() {}

            unsafe { cccr.modify(|r| r.init().set(false)) };
            while unsafe { cccr.read() }.init().get() {}
        }

        unsafe { cccr.modify(|r| r.init().set(true)) };
        while !unsafe { cccr.read() }.init().get() {}

        unsafe { cccr.modify(|r| r.cce().set(true).init().set(true)) };
    }
}
