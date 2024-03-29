use super::{PinIndex, PortIndex};
use core::convert::Infallible;

use super::{
    dynamic::PinModeError, marker, DynamicPin, ErasedPin, Output, PartiallyErasedPin, Pin,
};

use embedded_hal::digital::{
    ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin,
};

// Implementations for `Pin`
impl<const P: PortIndex, const N: PinIndex, MODE> ErrorType for Pin<P, N, MODE> {
    type Error = Infallible;
}

impl<const P: PortIndex, const N: PinIndex, MODE> OutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: PortIndex, const N: PinIndex, MODE> StatefulOutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|x| !x)
    }

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: PortIndex, const N: PinIndex, MODE> InputPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_high()
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}

// Implementations for `ErasedPin`
impl<MODE> ErrorType for ErasedPin<MODE> {
    type Error = core::convert::Infallible;
}

impl<MODE> OutputPin for ErasedPin<Output<MODE>> {
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for ErasedPin<Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<MODE> InputPin for ErasedPin<MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_high()
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self._is_low()
    }
}

// Implementations for `PartiallyErasedPin`
impl<const P: PortIndex, MODE> ErrorType for PartiallyErasedPin<P, MODE> {
    type Error = Infallible;
}

impl<const P: PortIndex, MODE> OutputPin for PartiallyErasedPin<P, Output<MODE>> {
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High);
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low);
        Ok(())
    }
}

impl<const P: PortIndex, MODE> StatefulOutputPin for PartiallyErasedPin<P, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(& mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: PortIndex, MODE> InputPin for PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_high()
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}

// Implementations for `DynamicPin
impl<const P: PortIndex, const N: PinIndex> ErrorType for DynamicPin<P, N> {
    type Error = PinModeError;
}

impl<const P: PortIndex, const N: PinIndex> OutputPin for DynamicPin<P, N> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }
}

impl<const P: PortIndex, const N: PinIndex> InputPin for DynamicPin<P, N> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_high()
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}
