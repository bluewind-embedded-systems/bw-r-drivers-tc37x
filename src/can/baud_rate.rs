// Many integer and float conversions are done in this file, we want to get rid of them
#![deny(clippy::as_conversions, clippy::float_cmp)]

use crate::util::F32Abs;

pub(super) const NBTP_NBRP_MSK: usize = 0x1ff;
pub(super) const NBTP_NTSEG1_MSK: usize = 0xff;
pub(super) const NBTP_NTSEG2_MSK: usize = 0x7f;

pub(super) const DBTP_DBRP_MSK: usize = 0x1f;
pub(super) const DBTP_DTSEG1_MSK: usize = 0x1f;
pub(super) const DBTP_DTSEG2_MSK: usize = 0xf;

pub(super) struct BestBaudRate {
    pub(super) tbaud: i32,
    pub(super) brp: i32,
}

pub(super) fn get_best_baud_rate(
    brp_msk: usize,
    tseg1_msk: usize,
    tseg2_msk: usize,
    module_freq: f32,
    baudrate: u32,
) -> BestBaudRate {
    /* search for best baudrate */

    let max_brp = brp_msk as i32 + 1;
    let min_brp = 1;

    let max_tbaud = (tseg1_msk as i32 + 1) + (tseg2_msk as i32 + 1) + 1;
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

    BestBaudRate {
        tbaud: best_tbaud,
        brp: best_brp,
    }
}

pub(super) fn get_best_sample_point(
    tseg1_msk: usize,
    tseg2_msk: usize,
    best_tbaud: i32,
    sample_point: u16,
) -> (i32, i32) {
    /* search for best sample point */

    let mut best_error = sample_point as f32 * 0.25; /* 25% tolerance in sample point as max error */
    let max_tseg1 = tseg1_msk as i32 + 1;
    let min_tseg1 = 3;
    let max_tseg2 = tseg2_msk as i32 + 1;
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

pub(super) fn get_best_sjw(best_tbaud: u32, best_tseg2: u32, sync_jump_width: u16) -> u32 {
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

pub(super) struct BitTiming {
    pub(super) brp: u16,
    pub(super) sjw: u8,
    pub(super) tseg1: u8,
    pub(super) tseg2: u8,
}

pub(super) fn calculate_bit_timing(
    module_freq: f32,
    baud_rate: u32,
    sample_point: u16,
    sjw: u16,
) -> BitTiming {
    /* Set values into node */
    let best = get_best_baud_rate(
        NBTP_NBRP_MSK,
        NBTP_NTSEG1_MSK,
        NBTP_NTSEG2_MSK,
        module_freq,
        baud_rate,
    );

    let (best_tseg1, best_tseg2) =
        get_best_sample_point(NBTP_NTSEG1_MSK, NBTP_NTSEG2_MSK, best.tbaud, sample_point);
    let best_sjw = get_best_sjw(best.tbaud as _, best_tseg2 as _, sjw);

    // TODO check this smell, why 1 is subtracted from these values?
    BitTiming {
        brp: best.brp as u16 - 1,
        sjw: best_sjw as u8 - 1,
        tseg1: best_tseg1 as u8 - 1,
        tseg2: best_tseg2 as u8 - 1,
    }
}

pub(super) fn calculate_fast_bit_timing(
    module_freq: f32,
    baud_rate: u32,
    sample_point: u16,
    sjw: u16,
) -> BitTiming {
    /* Set values into node */
    let best = get_best_baud_rate(
        DBTP_DBRP_MSK,
        DBTP_DTSEG1_MSK,
        DBTP_DTSEG2_MSK,
        module_freq,
        baud_rate,
    );

    let (best_tseg1, best_tseg2) =
        get_best_sample_point(DBTP_DTSEG1_MSK, DBTP_DTSEG2_MSK, best.tbaud, sample_point);
    let best_sjw = get_best_sjw(best.tbaud as _, best_tseg2 as _, sjw);

    // TODO check this smell, why 1 is subtracted from these values?
    BitTiming {
        brp: best.brp as u16 - 1,
        sjw: best_sjw as u8 - 1,
        tseg1: best_tseg1 as u8 - 1,
        tseg2: best_tseg2 as u8 - 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_best_baudrate() {
        let module_freq = 80000000.0;
        let baudrate = 500000;
        let best = get_best_baud_rate(
            NBTP_NBRP_MSK,
            NBTP_NTSEG1_MSK,
            NBTP_NTSEG2_MSK,
            module_freq,
            baudrate,
        );

        assert_eq!(best.tbaud, 20);
        assert_eq!(best.brp, 8);
    }

    #[test]
    fn test_get_best_sample_point() {
        let sample_point = 8000;
        let best_tbaud = 20;

        let (best_tseg1, best_tseg2) =
            get_best_sample_point(NBTP_NTSEG1_MSK, NBTP_NTSEG2_MSK, best_tbaud, sample_point);

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
