use super::can_node::{CanNode, NodeId};
use crate::{pac, scu};

#[derive(Default)]
pub struct CanModuleConfig {}

pub struct CanModule {
    inner: pac::can0::Can0,
}

impl CanModule {
    pub const fn new(_index: usize) -> Self {
        // TODO Use index
        Self { inner: pac::CAN0 }
    }

    pub fn init(self, _config: CanModuleConfig) -> Result<CanModule, ()> {
        if !self.is_enabled() {
            self.enable_module();
        }

        Ok(self)
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        !unsafe { self.inner.clc().read() }.diss().get()
    }

    pub fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { self.inner.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

    pub fn get_node(&mut self, node_id: NodeId) -> Result<CanNode, ()> {
        // Instead of dealing with lifetimes, we just create a new instance of CanModule
        // TODO This is not ideal, but it works for now
        let module = CanModule { inner: self.inner };
        Ok(CanNode::new(module, node_id))
    }
}
