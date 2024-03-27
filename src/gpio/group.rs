// TODO (alepez) Remove this warning suppression
#![allow(unused)]

use super::*;

/// Convert tuple or array of pins to output port
pub trait PinGroup {
    type Target;
    fn into_pin_group(self) -> Self::Target;
}

macro_rules! pin_group {
    ( $name:ident => $n:literal, ( $($i:tt),+ ), ( $($N:ident),+ )) => {
        pub struct $name<const P: PortIndex $(, const $N: PinIndex)+> (
            $(pub Pin<P, $N, Output<PushPull>>,)+
        );

        impl<const P: PortIndex $(, const $N: PinIndex)+> PinGroup for ($(Pin<P, $N, Output<PushPull>>),+) {
            type Target = $name<P $(, $N)+>;
            fn into_pin_group(self) -> Self::Target {
                $name($(self.$i),+)
            }
        }

        /// Wrapper for tuple of `Pin`s
        impl<const P: PortIndex $(, const $N: PinIndex)+> $name<P $(, $N)+> {
            const fn mask() -> u32 {
                0 $( | (1 << { $N }))+
            }
            const fn value_for_write_bsrr(val: u32) -> u32 {
                0 $( | (1 << (if val & (1 << $i) != 0 { $N } else { $N + 16 })))+
            }

            #[doc=concat!("Set/reset pins according to `", $n, "` lower bits")]
            #[inline(never)]
            pub fn write(&mut self, word: u32) {
                let port = unsafe { (*Gpio::<P>::ptr()) };
                let raw = Self::value_for_write_bsrr(word);
                unsafe {
                    port.omr().init(|mut r| r.set_raw(raw));
                }
            }

            /// Set all pins to `PinState::High`
            pub fn set_high(&mut self) {
                let port = unsafe { (*Gpio::<P>::ptr()) };
                let raw = Self::mask();
                unsafe {
                    port.omr().init(|mut r| r.set_raw(raw));
                }
            }

            /// Reset all pins to `PinState::Low`
            pub fn set_low(&mut self) {
                let port = unsafe { (*Gpio::<P>::ptr()) };
                let raw = Self::mask() << 16;
                unsafe {
                    port.omr().init(|mut r| r.set_raw(raw));
                }
            }

            /// Set all pins' state
            pub fn set_state(&mut self, states: [PinState; $n]) {
                let port = unsafe { (*Gpio::<P>::ptr()) };
                let raw = 0 $( | (1 << (if states[$i] == PinState::High { $N } else { $N + 16 })))+;
                unsafe {
                    port.omr().init(|mut r| r.set_raw(raw));
                }
            }
        }
    }
}

pin_group!(PinGroup2 => 2, (0, 1), (N0, N1));
pin_group!(PinGroup3 => 3, (0, 1, 2), (N0, N1, N2));
pin_group!(PinGroup4 => 4, (0, 1, 2, 3), (N0, N1, N2, N3));
pin_group!(PinGroup5 => 5, (0, 1, 2, 3, 4), (N0, N1, N2, N3, N4));
pin_group!(PinGroup6 => 6, (0, 1, 2, 3, 4, 5), (N0, N1, N2, N3, N4, N5));
pin_group!(PinGroup7 => 7, (0, 1, 2, 3, 4, 5, 6), (N0, N1, N2, N3, N4, N5, N6));
pin_group!(PinGroup8 => 8, (0, 1, 2, 3, 4, 5, 6, 7), (N0, N1, N2, N3, N4, N5, N6, N7));

/// Wrapper for array of `PartiallyErasedPin`s
pub struct PinArray<const P: PortIndex, const SIZE: usize>(pub [PEPin<P, Output<PushPull>>; SIZE]);

impl<const P: PortIndex, const SIZE: usize> PinGroup for [PEPin<P, Output<PushPull>>; SIZE] {
    type Target = PinArray<P, SIZE>;
    fn into_pin_group(self) -> Self::Target {
        PinArray(self)
    }
}

impl<const P: PortIndex, const SIZE: usize> PinArray<P, SIZE> {
    fn mask(&self) -> u32 {
        let mut msk = 0;
        for pin in &self.0 {
            msk |= 1 << pin.pin.0;
        }
        msk
    }

    #[allow(clippy::if_not_else)]
    fn value_for_write_bsrr(&self, val: u32) -> u32 {
        let mut msk = 0;
        for (idx, pin) in self.0.iter().enumerate() {
            let n = pin.pin.0;
            msk |= 1 << (if val & (1 << idx) != 0 { n } else { n + 16 });
        }
        msk
    }

    /// Set/reset pins according to `SIZE` lower bits
    #[inline(never)]
    pub fn write(&mut self, word: u32) {
        let port = unsafe { (*Gpio::<P>::ptr()) };
        let raw = self.value_for_write_bsrr(word);
        unsafe {
            port.omr().init(|mut r| r.set_raw(raw));
        }
    }

    /// Set all pins to `PinState::High`
    pub fn set_high(&mut self) {
        let port = unsafe { (*Gpio::<P>::ptr()) };
        let raw = self.mask();

        unsafe {
            port.omr().init(|mut r| r.set_raw(raw));
        }
    }

    /// Reset all pins to `PinState::Low`
    pub fn set_low(&mut self) {
        let port = unsafe { (*Gpio::<P>::ptr()) };
        let raw = self.mask() << 16;

        unsafe {
            port.omr().init(|mut r| r.set_raw(raw));
        }
    }

    /// Set all pins' state
    pub fn set_state(&mut self, states: [PinState; SIZE]) {
        let port = unsafe { (*Gpio::<P>::ptr()) };
        let mut raw = 0;

        for (pin, state) in self.0.iter().zip(states.into_iter()) {
            let (pclx, psx) = pcl_ps_from_state(state);
            raw |= pcl_ps_bits(pclx, psx, pin.pin.0.into());
        }

        unsafe {
            port.omr().init(|mut r| r.set_raw(raw));
        }
    }
}
