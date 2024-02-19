// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::{InterruptLine, Module0, Module1, Tos};
use crate::cpu::Priority;
use core::intrinsics::transmute;
use tc37x_pac::src::Can0Int0;
use tc37x_pac::{Reg, RW};

// Note: for simplicity, this wraps a value of Can0Int0 type, even if the
// underlying registers have different types in the PAC crate.
pub(crate) struct ServiceRequest(Reg<Can0Int0, RW>);

impl Module0 {
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        ServiceRequest(match line {
            InterruptLine::Line0 => tc37x_pac::SRC.can0int0(),
            InterruptLine::Line1 => unsafe { transmute(tc37x_pac::SRC.can0int1()) },
            InterruptLine::Line2 => unsafe { transmute(tc37x_pac::SRC.can0int2()) },
            InterruptLine::Line3 => unsafe { transmute(tc37x_pac::SRC.can0int3()) },
            InterruptLine::Line4 => unsafe { transmute(tc37x_pac::SRC.can0int4()) },
            InterruptLine::Line5 => unsafe { transmute(tc37x_pac::SRC.can0int5()) },
            InterruptLine::Line6 => unsafe { transmute(tc37x_pac::SRC.can0int6()) },
            InterruptLine::Line7 => unsafe { transmute(tc37x_pac::SRC.can0int7()) },
            InterruptLine::Line8 => unsafe { transmute(tc37x_pac::SRC.can0int8()) },
            InterruptLine::Line9 => unsafe { transmute(tc37x_pac::SRC.can0int9()) },
            InterruptLine::Line10 => unsafe { transmute(tc37x_pac::SRC.can0int10()) },
            InterruptLine::Line11 => unsafe { transmute(tc37x_pac::SRC.can0int11()) },
            InterruptLine::Line12 => unsafe { transmute(tc37x_pac::SRC.can0int12()) },
            InterruptLine::Line13 => unsafe { transmute(tc37x_pac::SRC.can0int13()) },
            InterruptLine::Line14 => unsafe { transmute(tc37x_pac::SRC.can0int14()) },
            InterruptLine::Line15 => unsafe { transmute(tc37x_pac::SRC.can0int15()) },
        })
    }
}

impl Module1 {
    pub(crate) fn service_request(line: InterruptLine) -> ServiceRequest {
        ServiceRequest(match line {
            InterruptLine::Line0 => unsafe { transmute(tc37x_pac::SRC.can1int0()) },
            InterruptLine::Line1 => unsafe { transmute(tc37x_pac::SRC.can1int1()) },
            InterruptLine::Line2 => unsafe { transmute(tc37x_pac::SRC.can1int2()) },
            InterruptLine::Line3 => unsafe { transmute(tc37x_pac::SRC.can1int3()) },
            InterruptLine::Line4 => unsafe { transmute(tc37x_pac::SRC.can1int4()) },
            InterruptLine::Line5 => unsafe { transmute(tc37x_pac::SRC.can1int5()) },
            InterruptLine::Line6 => unsafe { transmute(tc37x_pac::SRC.can1int6()) },
            InterruptLine::Line7 => unsafe { transmute(tc37x_pac::SRC.can1int7()) },
            InterruptLine::Line8 => unsafe { transmute(tc37x_pac::SRC.can1int8()) },
            InterruptLine::Line9 => unsafe { transmute(tc37x_pac::SRC.can1int9()) },
            InterruptLine::Line10 => unsafe { transmute(tc37x_pac::SRC.can1int10()) },
            InterruptLine::Line11 => unsafe { transmute(tc37x_pac::SRC.can1int11()) },
            InterruptLine::Line12 => unsafe { transmute(tc37x_pac::SRC.can1int12()) },
            InterruptLine::Line13 => unsafe { transmute(tc37x_pac::SRC.can1int13()) },
            InterruptLine::Line14 => unsafe { transmute(tc37x_pac::SRC.can1int14()) },
            InterruptLine::Line15 => unsafe { transmute(tc37x_pac::SRC.can1int15()) },
        })
    }
}

impl ServiceRequest {
    pub(crate) fn enable(&self, priority: Priority, tos: Tos) {
        let priority = u8::from(priority);
        let tos = u8::from(tos);

        // Set priority and type of service
        unsafe { self.0.modify(|r| r.srpn().set(priority).tos().set(tos)) };

        // Clear request
        unsafe { self.0.modify(|r| r.clrr().set(true)) };

        // Enable service request
        unsafe { self.0.modify(|r| r.sre().set(true)) };
    }
}
