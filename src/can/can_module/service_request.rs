use crate::can::{InterruptLine, Module0, Module1, Tos};
use crate::cpu::Priority;
use crate::pac::src::can::{can_can::CaNxInTy_SPEC, CanCan};
use crate::pac::{Reg, RW, SRC};

pub(crate) struct ServiceRequest(Reg<CaNxInTy_SPEC, RW>);

impl Module0 {
    #[inline(always)]
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        module_service_request(0, line)
    }
}

impl Module1 {
    #[inline(always)]
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        module_service_request(1, line)
    }
}

impl ServiceRequest {
    pub(crate) fn enable(&self, priority: Priority, tos: Tos) {
        let priority = u8::from(priority);
        let tos = u8::from(tos);

        // Set priority and type of service
        // SAFETY: FIXME Check Aurix manual, tos is in range [0, 3], bits 9:8, 15:14, 23:21, 31 are written with 0
        // TODO .tos() is only available in patched pac. If Infineon does not fix it, we need to use set_raw
        unsafe { self.0.modify(|r| r.srpn().set(priority).tos().set(tos)) };

        // Clear request
        // SAFETY: CLRR is a W bit, bits 9:8, 15:14, 23:21, 31 are written with 0
        unsafe { self.0.modify(|r| r.clrr().set(true)) };

        // Enable service request
        // SAFETY: SRE is a RW bit, bits 9:8, 15:14, 23:21, 31 are written with 0
        unsafe { self.0.modify(|r| r.sre().set(true)) };
    }
}

fn module_service_request(module_id: usize, interrupt_line: InterruptLine) -> ServiceRequest {
    let modules = SRC.can().can_can();

    // SAFETY: module_id is in range [0, 1] because
    let module: &CanCan = unsafe { modules.get_unchecked(module_id) };

    let line_index = usize::from(u8::from(interrupt_line));

    // SAFETY: line_index is in range [0, 15] because InterruptLine is an enum with 16 variants
    let x = unsafe { *module.canxinty().get_unchecked(line_index) };

    ServiceRequest(x)
}
