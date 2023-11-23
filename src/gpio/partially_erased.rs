use super::*;

pub use PartiallyErasedPin as PEPin;

/// Partially erased pin
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
pub struct PartiallyErasedPin<const P: usize, MODE> {
    // TODO (alepez) rename to pin_id
    pub(crate) i: PinId,
    _mode: PhantomData<MODE>,
}

impl<const P: usize, MODE> PartiallyErasedPin<P, MODE> {
    pub(crate) fn new(i: PinId) -> Self {
        Self {
            i,
            _mode: PhantomData,
        }
    }

    /// Convert partially type erased pin to `Pin` with fixed type
    pub fn restore<const N: usize>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.i.0, N);
        Pin::new()
    }
}

impl<const P: usize, MODE> fmt::Debug for PartiallyErasedPin<P, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}({})<{}>",
            P,
            self.i.0,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: usize, MODE> defmt::Format for PartiallyErasedPin<P, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P{}({})<{}>",
            P,
            self.i,
            crate::stripped_type_name::<MODE>()
        );
    }
}

impl<const P: usize, MODE> PinExt for PartiallyErasedPin<P, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> PinId {
        self.i
    }
    #[inline(always)]
    fn port_id(&self) -> PortId {
        PortId(P)
    }
}

impl<const P: usize, MODE> PartiallyErasedPin<P, Output<MODE>> {
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
        set_output_pin_state(port, self.i, state);
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub(crate) fn _is_set_high(&self) -> bool {
        let port = &(unsafe { *Gpio::<P>::ptr() });
        unsafe {
            match self.i.0 {
                0 => port.out().read().p0().get(),
                1 => port.out().read().p1().get(),
                2 => port.out().read().p2().get(),
                3 => port.out().read().p3().get(),
                4 => port.out().read().p4().get(),
                5 => port.out().read().p5().get(),
                6 => port.out().read().p6().get(),
                7 => port.out().read().p7().get(),
                8 => port.out().read().p8().get(),
                9 => port.out().read().p9().get(),
                10 => port.out().read().p10().get(),
                11 => port.out().read().p11().get(),
                12 => port.out().read().p12().get(),
                13 => port.out().read().p13().get(),
                14 => port.out().read().p14().get(),
                15 => port.out().read().p15().get(),
                _ => unreachable!(),
            }
        }
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub(crate) fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        // TODO (alepez)
        // unsafe { (*Gpio::<P>::ptr()).odr.read().bits() & (1 << self.i) == 0 }
        todo!()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        let port = &unsafe { (*Gpio::<P>::ptr()) };
        toggle_output_pin_state(port, self.i)
    }
}

impl<const P: usize, MODE> PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        let port = &(unsafe { *Gpio::<P>::ptr() });
        unsafe {
            match self.i.0 {
                0 => port.r#in().read().p0().get(),
                1 => port.r#in().read().p1().get(),
                2 => port.r#in().read().p2().get(),
                3 => port.r#in().read().p3().get(),
                4 => port.r#in().read().p4().get(),
                5 => port.r#in().read().p5().get(),
                6 => port.r#in().read().p6().get(),
                7 => port.r#in().read().p7().get(),
                8 => port.r#in().read().p8().get(),
                9 => port.r#in().read().p9().get(),
                10 => port.r#in().read().p10().get(),
                11 => port.r#in().read().p11().get(),
                12 => port.r#in().read().p12().get(),
                13 => port.r#in().read().p13().get(),
                14 => port.r#in().read().p14().get(),
                15 => port.r#in().read().p15().get(),
                _ => unreachable!(),
            }
        }
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}

impl<const P: usize, MODE> From<PartiallyErasedPin<P, MODE>> for ErasedPin<MODE> {
    /// Partially erased pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: PartiallyErasedPin<P, MODE>) -> Self {
        ErasedPin::new(PortId(P), p.i)
    }
}
