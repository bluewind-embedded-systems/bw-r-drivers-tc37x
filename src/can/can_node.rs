use super::baud_rate::*;
use super::can_module::ClockSource;
use super::frame::Frame;
use super::CanModule;
use crate::util::wait_nop;

// TODO Default values are not valid
#[derive(Default)]
pub struct BaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
}

// TODO Default values are not valid
#[derive(Default)]
pub struct FastBaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
    pub transceiver_delay_offset: u8,
}

#[derive(PartialEq, Debug, Default)]
pub enum FrameMode {
    #[default]
    Standard,
    FdLong,
    FdLongAndFast,
}

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
    pub calculate_bit_timing_values: bool,
    pub baud_rate: BaudRate,
    pub fast_baud_rate: FastBaudRate,
    pub frame_mode: FrameMode,
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

        self.configure_baud_rate(config.calculate_bit_timing_values, &config.baud_rate);

        // for CAN FD frames, set fast baudrate
        if config.frame_mode != FrameMode::Standard {
            self.configure_fast_baud_rate(
                config.calculate_bit_timing_values,
                &config.fast_baud_rate,
            );
        }

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

    fn configure_baud_rate(&self, calculate_bit_timing_values: bool, baud_rate: &BaudRate) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            let timing = calculate_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
            self.set_bit_timing(timing);
        } else {
            self.set_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar,
            )
        }
    }

    fn configure_fast_baud_rate(
        &self,
        calculate_bit_timing_values: bool,
        baud_rate: &FastBaudRate,
    ) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            self.set_fast_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
        } else {
            self.set_fast_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar as u8,
            );
        }

        if baud_rate.transceiver_delay_offset != 0 {
            self.set_transceiver_delay_compensation_offset(baud_rate.transceiver_delay_offset);
        }
    }

    fn set_bit_timing(&self, timing: BitTiming) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nbrp()
                    .set(timing.brp)
                    .nsjw()
                    .set(timing.sjw)
                    .ntseg1()
                    .set(timing.tseg1)
                    .ntseg2()
                    .set(timing.tseg2)
            })
        }
    }

    fn set_bit_timing_values(&self, sjw: u8, time_segment2: u8, time_segment1: u8, prescaler: u16) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nsjw()
                    .set(sjw)
                    .ntseg1()
                    .set(time_segment1)
                    .ntseg2()
                    .set(time_segment2)
                    .nbrp()
                    .set(prescaler)
            })
        };
    }

    fn set_fast_bit_timing(&self, module_freq: f32, baudrate: u32, sample_point: u16, sjw: u16) {
        let timing = calculate_fast_bit_timing(module_freq, baudrate, sample_point, sjw);

        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dbrp()
                    .set(timing.brp.try_into().unwrap())
                    .dsjw()
                    .set(timing.sjw)
                    .dtseg1()
                    .set(timing.tseg1)
                    .dtseg2()
                    .set(timing.tseg2)
            })
        }
    }

    pub fn set_fast_bit_timing_values(
        &self,
        sjw: u8,
        time_segment2: u8,
        time_segment1: u8,
        prescaler: u8,
    ) {
        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dsjw()
                    .set(sjw)
                    .dtseg1()
                    .set(time_segment1)
                    .dtseg2()
                    .set(time_segment2)
                    .dbrp()
                    .set(prescaler)
            })
        };
    }

    pub fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
        unsafe { self.inner.dbtp().modify(|r| r.tdc().set(true)) };
        unsafe { self.inner.tdcr().modify(|r| r.tdco().set(delay)) };
    }
}
