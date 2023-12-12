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

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
    pub calculate_bit_timing_values: bool,
    pub baud_rate: BaudRate,
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
            self.set_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
        } else {
            self.set_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar,
            )
        }
    }

    fn set_bit_timing(
        &self,
        module_freq: f32,
        baudrate: u32,
        sample_point: u16,
        sync_jump_width: u16,
    ) {
        /* Set values into node */
        let (best_tbaud, best_brp) = get_best_baud_rate::<
            NBTP_NBRP_MSK,
            NBTP_NTSEG1_MSK,
            NBTP_NTSEG2_MSK,
        >(module_freq, baudrate);

        let (best_tseg1, best_tseg2) =
            get_best_sample_point::<NBTP_NTSEG1_MSK, NBTP_NTSEG2_MSK>(best_tbaud, sample_point);
        let best_sjw = get_best_sjw(best_tbaud, best_tseg2, sync_jump_width);

        #[cfg(feature = "log")]
        defmt::debug!(
            "brp:{}, sjw:{}, tseg1:{}, tseg2:{}",
            (best_brp - 1) as u16,
            (best_sjw - 1) as u8,
            (best_tseg1 - 1) as u8,
            (best_tseg2 - 1) as u8
        );

        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nbrp()
                    .set((best_brp - 1) as _)
                    .nsjw()
                    .set((best_sjw - 1) as _)
                    .ntseg1()
                    .set((best_tseg1 - 1) as _)
                    .ntseg2()
                    .set((best_tseg2 - 1) as _)
            })
        }
    }

    fn set_bit_timing_values(
        &self,
        sync_jump_width: u8,
        time_segment2: u8,
        time_segment1: u8,
        prescaler: u16,
    ) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nsjw()
                    .set(sync_jump_width)
                    .ntseg1()
                    .set(time_segment1)
                    .ntseg2()
                    .set(time_segment2)
                    .nbrp()
                    .set(prescaler)
            })
        };
    }
}
