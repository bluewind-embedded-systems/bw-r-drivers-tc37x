use crate::tracing;

#[allow(unreachable_code)]
pub(crate) unsafe fn load_modify_store(addr: *mut u32, v: u32, m: u32) {
    #[cfg(feature = "tracing")]
    return tracing::load_modify_store(addr as usize, v as u64 | ((m as u64) << 32));

    #[cfg(target_arch = "tricore")]
    return unsafe {
        core::arch::tricore::intrinsics::__ldmst(addr, v, m);
    };

    panic!("unsupported architecture");
}

// TODO
// pub(crate) unsafe fn write_volatile(addr: usize, len: usize, val: u64) {
//     #[cfg(feature = "tracing")]
//     return tracing::write_volatile(addr, len, val)
//
//     panic!("unsupported architecture");
// }

// TODO
// pub(crate) unsafe fn read_volatile(addr: usize, len: usize) -> u64 {
//     #[cfg(feature = "tracing")]
//     return tracing::read_volatile(addr, len);
//
//     panic!("unsupported architecture");
// }