use crate::can::{
    BitTimingConfig, ClockSource, FastBitTimingConfig, FrameMode, Interrupt, InterruptGroup,
    InterruptLine, MessageRAM, Pins, RxConfig, Tos, TxConfig,
};
use crate::cpu::Priority;

pub struct NodeInterruptConfig {
    pub interrupt_group: InterruptGroup,
    pub interrupt: Interrupt,
    pub line: InterruptLine,
    pub priority: Priority,
    pub tos: Tos,
}

pub struct NodeConfig<M, N> {
    pub clock_source: ClockSource,
    pub baud_rate: BitTimingConfig,
    pub fast_baud_rate: FastBitTimingConfig,
    pub transceiver_delay_offset: u8,
    pub frame_mode: FrameMode,
    pub tx: Option<TxConfig>,
    pub rx: Option<RxConfig>,
    pub message_ram: MessageRAM,
    pub pins: Option<Pins<M, N>>,
}

// Note: the Default trait implementation must be explicitly defined because
// the derive macro needs all generic parameters to implement Default, even
// if it is not necessary.
impl<M, N> Default for NodeConfig<M, N> {
    fn default() -> Self {
        Self {
            clock_source: Default::default(),
            baud_rate: Default::default(),
            fast_baud_rate: Default::default(),
            transceiver_delay_offset: 0,
            frame_mode: Default::default(),
            tx: None,
            rx: None,
            message_ram: Default::default(),
            pins: None,
        }
    }
}
