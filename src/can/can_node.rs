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
        let (best_tbaud, best_brp) = get_best_baudrate::<
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

fn get_best_baudrate<const BRP_MSK: usize, const TSEG1_MSK: usize, const TSEG2_MSK: usize>(
    module_freq: f32,
    baudrate: u32,
) -> (i32, i32) {
    /* search for best baudrate */

    let max_brp = BRP_MSK as i32 + 1;
    let min_brp = 1;

    let max_tbaud = (TSEG1_MSK as i32 + 1) + (TSEG2_MSK as i32 + 1) + 1;
    let min_tbaud = 8;

    let mut best_error = baudrate as f32;
    let mut best_brp = 1;
    let mut best_tbaud = 8;
    let mut tmp_brp = 1;
    let mut tmp_tbaud: i32 = 0;

    while tmp_brp <= max_brp {
        let f_quanta = module_freq / tmp_brp as f32;
        tmp_tbaud = (f_quanta / baudrate as f32) as _;

        if tmp_tbaud == 0 {
            break; /* to avoid division by 0 */
        }

        let temp_baudrate = f_quanta / tmp_tbaud as f32;
        let error = (temp_baudrate - baudrate as f32).abs();

        if tmp_tbaud < min_tbaud {
            break; /* below the minimum allowed limits, break is required otherwise TSEG1 and TSEG2 may result in negitive values */
        }

        if (tmp_tbaud <= max_tbaud) && (best_error >= error) {
            best_brp = tmp_brp;
            best_tbaud = tmp_tbaud;
            best_error = error;

            if (tmp_tbaud <= 20) && (error < 0.1) {
                break; /* optimal condition */
            }
        }
        tmp_brp += 1;
    }

    if (best_brp == 0) && (tmp_brp == (max_brp + 1)) {
        // force max brp and tbaud
        best_brp = max_brp;
        best_tbaud = max_tbaud;
    }

    if (best_brp == 0) && (tmp_tbaud < min_tbaud) {
        // force min brp and tbaud
        best_brp = min_brp;
        best_tbaud = min_tbaud;
    }

    (best_tbaud, best_brp)
}

fn get_best_sample_point<const TSEG1_MSK: usize, const TSEG2_MSK: usize>(
    best_tbaud: i32,
    sample_point: u16,
) -> (i32, i32) {
    /* search for best sample point */

    let mut best_error = sample_point as f32 * 0.25; /* 25% tolerance in sample point as max error */
    let max_tseg1 = TSEG1_MSK as i32 + 1;
    let min_tseg1 = 3;
    let max_tseg2 = TSEG2_MSK as i32 + 1;
    let min_tseg2 = 2;

    let max_tseg1 = if best_tbaud < max_tseg1 {
        best_tbaud
    } else {
        max_tseg1
    };
    let mut best_tseg1 = max_tseg1;

    for temp_tseg1 in (min_tseg1..=max_tseg1).rev() {
        let temp_sample_point = ((temp_tseg1 + 1) * 10000) / best_tbaud;
        let error = temp_sample_point - sample_point as i32;
        let error = if error < 0 { -error } else { error };

        if best_error > error as _ {
            best_tseg1 = temp_tseg1;
            best_error = error as _;
        }

        if temp_sample_point < sample_point as _ {
            /*least possible error */
            break; /* least possible error has already occured */
        }
    }

    let best_tseg2 = if (best_tbaud - best_tseg1 - 1) > max_tseg2 {
        // force max tseg2
        max_tseg2
    } else {
        best_tbaud - best_tseg1 - 1
    };

    let best_tseg2 = if best_tseg2 < min_tseg2 {
        // force min tseg2
        min_tseg2
    } else {
        best_tseg2
    };

    (best_tseg1, best_tseg2)
}

fn get_best_sjw(best_tbaud: i32, best_tseg2: i32, sync_jump_width: u16) -> i32 {
    let mut best_sjw = 0;
    let mut best_error = 10000.0;

    for temp_sjw in 1..=best_tseg2 {
        let temp_sync_jump_width = (temp_sjw as f32 * 10000.0) / best_tbaud as f32;
        let error = (temp_sync_jump_width - sync_jump_width as f32).abs();

        if best_error > error {
            best_sjw = temp_sjw;
            best_error = error;
        }
    }
    best_sjw
}

const NBTP_NBRP_MSK: usize = 0x1ff;
const NBTP_NTSEG1_MSK: usize = 0xff;
const NBTP_NTSEG2_MSK: usize = 0x7f;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_best_baudrate() {
        let module_freq = 80000000.0;
        let baudrate = 500000;
        let (best_tbaud, best_brp) = get_best_baudrate::<
            NBTP_NBRP_MSK,
            NBTP_NTSEG1_MSK,
            NBTP_NTSEG2_MSK,
        >(module_freq, baudrate);

        assert_eq!(best_tbaud, 20);
        assert_eq!(best_brp, 8);
    }

    #[test]
    fn test_get_best_sample_point() {
        let sample_point = 8000;
        let best_tbaud = 20;

        let (best_tseg1, best_tseg2) =
            get_best_sample_point::<NBTP_NTSEG1_MSK, NBTP_NTSEG2_MSK>(best_tbaud, sample_point);

        assert_eq!(best_tseg1, 15);
        assert_eq!(best_tseg2, 4);
    }

    #[test]
    fn test_get_best_sjw() {
        let best_tboud = 20;
        let best_tseg2 = 4;
        let sync_jump_width = 3;
        let best_sjw = get_best_sjw(best_tboud, best_tseg2, sync_jump_width);

        assert_eq!(best_sjw, 1);
    }
}
