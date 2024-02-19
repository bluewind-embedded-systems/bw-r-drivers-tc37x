use crate::can::{
    BitTimingConfig, ClockSource, FastBitTimingConfig, FrameMode, Interrupt, InterruptGroup,
    InterruptLine, Tos,
};
use crate::cpu::Priority;

pub struct NodeInterruptConfig {
    pub interrupt_group: InterruptGroup,
    pub interrupt: Interrupt,
    pub line: InterruptLine,
    pub priority: Priority,
    pub tos: Tos,
}

#[derive(Default)]
pub struct NodeConfig {
    pub clock_source: ClockSource,
    pub baud_rate: BitTimingConfig,
    pub fast_baud_rate: FastBitTimingConfig,
    pub transceiver_delay_offset: u8,
    pub frame_mode: FrameMode,
}
