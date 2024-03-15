use crate::can::{InterruptLine, Module0, Module1, Tos};
use crate::cpu::Priority;
use tc37x::src::can::can_can::CaNxInTy_SPEC;
use tc37x::{Reg, RW};

pub(crate) struct ServiceRequest(Reg<CaNxInTy_SPEC, RW>);

impl Module0 {
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        let line_index = usize::from(line as u8);
        let x = tc37x::SRC.can().can_can()[0].canxinty()[line_index];
        ServiceRequest(x)
    }
}

impl Module1 {
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        let line_index = usize::from(line as u8);
        let x = tc37x::SRC.can().can_can()[1].canxinty()[line_index];
        ServiceRequest(x)
    }
}

impl ServiceRequest {
    pub(crate) fn enable(&self, priority: Priority, tos: Tos) {
        let priority = u8::from(priority);
        let tos = u8::from(tos);

        // Set priority and type of service
        // SAFETY: FIXME Check Aurix manual, tos is in range [0, 3], bits 9:8, 15:14, 23:21, 31 are written with 0
        // TODO .tos() is only available in patched pac. If Infineon does not fix it, we need to use set_raw
        unsafe { self.0.modify(|r| r.srpn().set(priority).tos().set(tos.into())) };

        // Clear request
        // SAFETY: CLRR is a W bit, bits 9:8, 15:14, 23:21, 31 are written with 0
        unsafe { self.0.modify(|r| r.clrr().set(1u8.into())) };

        // Enable service request
        // SAFETY: SRE is a RW bit, bits 9:8, 15:14, 23:21, 31 are written with 0
        unsafe { self.0.modify(|r| r.sre().set(1u8.into())) };
    }
}
