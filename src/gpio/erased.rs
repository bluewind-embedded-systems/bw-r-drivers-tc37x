// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

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
    pub fn restore<const P: PortIndex, const N: PinIndex>(self) -> Pin<P, N, MODE> {
        assert_eq!(self.port_id().0, P);
        assert_eq!(self.pin_id().0, N);
        Pin::new()
    }

    #[inline]
    pub(crate) unsafe fn block(&self) -> &AnyPort {
        // This function uses pointer arithmetic instead of branching to be more efficient
        //
        // The logic relies on the following assumptions:
        //
        // - PORT_00 register is available on all chips
        // - all gpio register blocks have the same layout
        // - consecutive gpio register blocks have the same offset between them: 0x100 (256)
        // - ErasedPin::new was called with a valid port

        use crate::pac::p00::P00 as Port;
        use crate::pac::P00 as PORT;

        const PORT_REGISTER_OFFSET: usize = 0x100;

        #[allow(clippy::useless_conversion)]
        let port_index: usize = self.port_id().0.into();
        let offset = PORT_REGISTER_OFFSET * port_index;
        let block_ptr = unsafe { (&PORT as *const Port).add(offset) };

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
        if self._is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        let port = unsafe { self.block() };
        pin_set_state(port, self.pin, state);
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub(crate) fn _is_set_high(&self) -> bool {
        let port = unsafe { self.block() };
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
        let port = unsafe { self.block() };
        pin_toggle_state(port, self.pin)
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
        pin_input_is_high(port, self.pin)
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}
