// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use super::*;

pub use PartiallyErasedPin as PEPin;

/// Partially erased pin
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
pub struct PartiallyErasedPin<const P: PortIndex, MODE> {
    pub(crate) pin: PinId,
    _mode: PhantomData<MODE>,
}

impl<const P: PortIndex, MODE> PartiallyErasedPin<P, MODE> {
    pub(crate) fn new(i: PinId) -> Self {
        Self {
            pin: i,
            _mode: PhantomData,
        }
    }

    /// Convert partially type erased pin to `Pin` with fixed type
    pub fn restore<const N: PinIndex>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.pin.0, N);
        Pin::new()
    }
}

impl<const P: PortIndex, MODE> fmt::Debug for PartiallyErasedPin<P, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}({})<{}>",
            P,
            self.pin.0,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: PortIndex, MODE> defmt::Format for PartiallyErasedPin<P, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P{}({})<{}>",
            P,
            self.pin,
            crate::stripped_type_name::<MODE>()
        );
    }
}

impl<const P: PortIndex, MODE> PinExt for PartiallyErasedPin<P, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> PinId {
        self.pin
    }
    #[inline(always)]
    fn port_id(&self) -> PortId {
        PortId(P)
    }
}

impl<const P: PortIndex, MODE> PartiallyErasedPin<P, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self.set_state(PinState::High)
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self.set_state(PinState::Low)
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    fn _get_state(&self) -> PinState {
        if self._is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        let port = &unsafe { (*Gpio::<P>::ptr()) };
        pin_set_state(port, self.pin, state);
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub(crate) fn _is_set_high(&self) -> bool {
        let port = &(unsafe { *Gpio::<P>::ptr() });
        pin_output_is_high(port, self.pin)
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub(crate) fn _is_set_low(&self) -> bool {
        !self._is_set_high()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        let port = &unsafe { (*Gpio::<P>::ptr()) };
        pin_toggle_state(port, self.pin)
    }
}

impl<const P: PortIndex, MODE> PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        let port = &(unsafe { *Gpio::<P>::ptr() });
        pin_input_is_high(port, self.pin)
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}

impl<const P: PortIndex, MODE> From<PartiallyErasedPin<P, MODE>> for ErasedPin<MODE> {
    /// Partially erased pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: PartiallyErasedPin<P, MODE>) -> Self {
        ErasedPin::new(PortId(P), p.pin)
    }
}
