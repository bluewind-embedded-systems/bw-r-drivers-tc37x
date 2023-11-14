pub use ErasedPin as EPin;

use super::*;

/// Fully erased pin
///
/// `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
pub struct ErasedPin<MODE> {
    // Bits 0-3: Pin, Bits 4-7: Port
    pin_port: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> fmt::Debug for ErasedPin<MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P({}{})<{}>",
            self.port_id(),
            self.pin_id(),
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
    fn pin_id(&self) -> u8 {
        self.pin_port & 0x0f
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        self.pin_port >> 4
    }
}

impl<MODE> ErasedPin<MODE> {
    pub(crate) fn from_pin_port(pin_port: u8) -> Self {
        Self {
            pin_port,
            _mode: PhantomData,
        }
    }
    pub(crate) fn into_pin_port(self) -> u8 {
        self.pin_port
    }
    pub(crate) fn new(port: u8, pin: u8) -> Self {
        Self {
            pin_port: port << 4 | pin,
            _mode: PhantomData,
        }
    }

    /// Convert type erased pin to `Pin` with fixed type
    pub fn restore<const P: usize, const N: u8>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.port_id(), P as u8 - b'A');
        assert_eq!(self.pin_id(), N);
        Pin::new()
    }

    #[inline]
    pub(crate) unsafe fn block(&self) -> &crate::pac::port_00::Port00 {
        // This function uses pointer arithmetic instead of branching to be more efficient

        // The logic relies on the following assumptions:
        // - PORT_00 register is available on all chips
        // - all gpio register blocks have the same layout
        // - consecutive gpio register blocks have the same offset between them: 0x100 (256)
        // - ErasedPin::new was called with a valid port

        // FIXME could be calculated after const_raw_ptr_to_usize_cast stabilization #51910
        const GPIO_REGISTER_OFFSET: usize = 0x100;

        let offset = GPIO_REGISTER_OFFSET * self.port_id() as usize;

        let block_ptr = (&crate::pac::PORT_00 as *const crate::pac::port_00::Port00).add(offset);

        unsafe { &*block_ptr }
    }
}

impl<MODE> ErasedPin<Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        // TODO (alepez)
        // unsafe { self.block().bsrr.write(|w| w.bits(1 << self.pin_id())) };
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        // TODO (alepez)
        // unsafe {
        //     self.block()
        //         .bsrr
        //         .write(|w| w.bits(1 << (self.pin_id() + 16)))
        // };
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
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
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
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        // TODO (alepez)
        // self.block().idr.read().bits() & (1 << self.pin_id()) == 0
        todo!()
    }
}
