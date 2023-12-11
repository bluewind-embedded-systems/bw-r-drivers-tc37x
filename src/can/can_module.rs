use crate::{pac, scu};
use super::can_node::CanNode;

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

    pub fn get_node(&mut self, _node_id: usize) -> Result<CanNode, ()> {
        // TODO
        Ok(CanNode)
    }
}
