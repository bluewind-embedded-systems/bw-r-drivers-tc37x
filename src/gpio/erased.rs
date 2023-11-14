pub use ErasedPin as EPin;

use super::*;

/// Fully erased pin
///
/// `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
pub struct ErasedPin<MODE> {
    // Pin index
    pin: PinId,
    // Port id
    port: PortId,
    _mode: PhantomData<MODE>,
}

impl<MODE> fmt::Debug for ErasedPin<MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P({}{})<{}>",
            self.port_id().0,
            self.pin_id().0,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<MODE> defmt::Format for ErasedPin<MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P({}{})<{}>",
            self.port_id(),
            self.pin_id(),
            crate::stripped_type_name::<MODE>()
        );
    }
}

impl<MODE> PinExt for ErasedPin<MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> PinId {
        self.pin
    }
    #[inline(always)]
    fn port_id(&self) -> PortId {
        self.port
    }
}

impl<MODE> ErasedPin<MODE> {
    pub(crate) fn new(port: PortId, pin: PinId) -> Self {
        Self {
            port,
            pin,
            _mode: PhantomData,
        }
    }

    /// Convert type erased pin to `Pin` with fixed type
    pub fn restore<const P: usize, const N: usize>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.port_id().0, P);
        assert_eq!(self.pin_id().0, N);
        Pin::new()
    }

    #[inline]
    pub(crate) unsafe fn block(&self) -> &crate::pac::port_00::Port00 {
        // This function uses pointer arithmetic instead of branching to be more efficient
        //
        // The logic relies on the following assumptions:
        //
        // - PORT_00 register is available on all chips
        // - all gpio register blocks have the same layout
        // - consecutive gpio register blocks have the same offset between them: 0x100 (256)
        // - ErasedPin::new was called with a valid port

        use crate::pac::port_00::Port00;
        use crate::pac::PORT_00;

        const PORT_REGISTER_OFFSET: usize = 0x100;

        let offset = PORT_REGISTER_OFFSET * self.port_id().0 as usize;
        let block_ptr = unsafe { (&PORT_00 as *const Port00).add(offset) };

        unsafe { &*block_ptr }
    }
}

impl<MODE> ErasedPin<Output<MODE>> {
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
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        let state = to_pcl_ps_bits(self.pin.0, &state);
        unsafe {
            self.block().omr().init(|mut r| r.set_raw(state));
        };
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        // TODO (alepez) there's no way to read the output state from registers
        todo!()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        // TODO (alepez) in tc37x toggle is possible without knowing the state
    }
}

impl<MODE> ErasedPin<MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        let port = unsafe { self.block() };
        unsafe {
            match self.pin.0 {
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
