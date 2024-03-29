// Many integer and float conversions are done in this file, we want to get rid of them
// TODO #![warn(clippy::as_conversions)]

#![allow(clippy::float_arithmetic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

use crate::log::info;
// The following import is needed when f32::abs is not available (tricore toolchain)
#[allow(unused_imports)]
use crate::util::F32Abs;

/// CAN bit timing configuration
pub enum BitTimingConfig {
    Auto(AutoBitTiming),
    Manual(NominalBitTiming),
}

impl Default for BitTimingConfig {
    fn default() -> Self {
        Self::Auto(AutoBitTiming::default())
    }
}

/// Fast CAN bit timing configuration
pub enum FastBitTimingConfig {
    Auto(AutoBitTiming),
    Manual(DataBitTiming),
}

impl Default for FastBitTimingConfig {
    fn default() -> Self {
        Self::Auto(AutoBitTiming::default())
    }
}

// TODO Default values are not valid
/// Automatic bit timing configuration
#[derive(Default)]
pub struct AutoBitTiming {
    /// Baud rate in bps
    pub baud_rate: u32,
    /// Sample point in 1/10th of a percent (e.g. 8000 = 80%)
    pub sample_point: u16,
    /// Synchronization jump width in time quanta
    pub sync_jump_width: u16,
}

pub(super) const NBTP_NBRP_MSK: i32 = 0x1ff;
pub(super) const NBTP_NTSEG1_MSK: i32 = 0xff;
pub(super) const NBTP_NTSEG2_MSK: i32 = 0x7f;

pub(super) const DBTP_DBRP_MSK: i32 = 0x1f;
pub(super) const DBTP_DTSEG1_MSK: i32 = 0x1f;
pub(super) const DBTP_DTSEG2_MSK: i32 = 0xf;

pub(super) struct BestBaudRate {
    pub(super) tbaud: i32,
    pub(super) brp: i32,
}

pub(super) fn get_best_baud_rate(
    brp_msk: i32,
    tseg1_msk: i32,
    tseg2_msk: i32,
    module_freq: f32,
    baudrate: u32,
) -> BestBaudRate {
    // Search for best baudrate

    let max_brp = brp_msk + 1;
    let min_brp = 1;

    let max_tbaud = (tseg1_msk + 1) + (tseg2_msk + 1) + 1;
    let min_tbaud = 8;

    let mut best_error = baudrate as f32;
    let mut best_brp = 1;
    let mut best_tbaud = 8;
    let mut tmp_brp = 1;
    let mut tmp_tbaud: i32 = 0;

    while tmp_brp <= max_brp {
        let f_quanta = module_freq / tmp_brp as f32;
        tmp_tbaud = (f_quanta / baudrate as f32) as i32;

        if tmp_tbaud == 0 {
            // Avoid division by 0
            break;
        }

        let temp_baudrate = f_quanta / tmp_tbaud as f32;
        let error = (temp_baudrate - baudrate as f32).abs();

        if tmp_tbaud < min_tbaud {
            // Below the minimum allowed limits, break is required otherwise
            // TSEG1 and TSEG2 may result in negative values
            break;
        }

        if (tmp_tbaud <= max_tbaud) && (best_error >= error) {
            best_brp = tmp_brp;
            best_tbaud = tmp_tbaud;
            best_error = error;

            if (tmp_tbaud <= 20) && (error < 0.1) {
                // Optimal condition
                break;
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
    tseg1_msk: i32,
    tseg2_msk: i32,
    best_tbaud: i32,
    sample_point: u16,
) -> (i32, i32) {
    // Search for best sample point

    // 25% tolerance in sample point as max error
    let mut best_error = f32::from(sample_point) * 0.25;
    let max_tseg1 = tseg1_msk + 1;
    let min_tseg1 = 3;
    let max_tseg2 = tseg2_msk + 1;
    let min_tseg2 = 2;

    let max_tseg1 = if best_tbaud < max_tseg1 {
        best_tbaud
    } else {
        max_tseg1
    };
    let mut best_tseg1 = max_tseg1;

    for temp_tseg1 in (min_tseg1..=max_tseg1).rev() {
        let temp_sample_point = ((temp_tseg1 + 1) * 10000) / best_tbaud;
        let error = temp_sample_point - i32::from(sample_point);
        let error = if error < 0 { -error } else { error };

        if best_error > error as f32 {
            best_tseg1 = temp_tseg1;
            best_error = error as f32;
        }

        if temp_sample_point < i32::from(sample_point) {
            // Least possible error has already occurred
            break;
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
        let error = (temp_sync_jump_width - f32::from(sync_jump_width)).abs();

        if best_error > error {
            best_sjw = temp_sjw;
            best_error = error;
        }
    }
    best_sjw
}

/// Nominal CAN bit timing
#[derive(Debug, Clone, Copy)]
pub struct NominalBitTiming {
    pub(super) brp: u16,
    pub(super) sjw: u8,
    pub(super) tseg1: u8,
    pub(super) tseg2: u8,
}

/// Data CAN bit timing
#[derive(Debug, Clone, Copy)]
pub struct DataBitTiming {
    pub(super) brp: u8,
    pub(super) sjw: u8,
    pub(super) tseg1: u8,
    pub(super) tseg2: u8,
}

pub(super) fn calculate_bit_timing(
    module_freq: f32,
    baud_rate: u32,
    sample_point: u16,
    sjw: u16,
) -> NominalBitTiming {
    info!(
        "module_freq: {}, baud_rate: {}, sample_point: {}, sync_jump_with: {}",
        module_freq, baud_rate, sample_point, sjw
    );

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
    let best_sjw = get_best_sjw(best.tbaud as u32, best_tseg2 as u32, sjw);

    NominalBitTiming {
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
) -> DataBitTiming {
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
    let best_sjw = get_best_sjw(best.tbaud as u32, best_tseg2 as u32, sjw);

    DataBitTiming {
        brp: best.brp as u8 - 1,
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
