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

pub struct NodeConfig {
    pub clock_source: ClockSource,
    pub baud_rate: BitTimingConfig,
    pub fast_baud_rate: FastBitTimingConfig,
    pub transceiver_delay_offset: u8,
    pub frame_mode: FrameMode,
}

// Note: the Default trait implementation must be explicitly defined because
// the derive macro needs all generic parameters to implement Default, even
// if it is not necessary.
impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            clock_source: Default::default(),
            baud_rate: Default::default(),
            fast_baud_rate: Default::default(),
            transceiver_delay_offset: 0,
            frame_mode: Default::default(),
        }
    }
}
