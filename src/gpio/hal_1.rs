use core::convert::Infallible;

use super::{
    dynamic::PinModeError, marker, DynamicPin, ErasedPin, Output, PartiallyErasedPin, Pin,
};

use embedded_hal::digital::{
    ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin, ToggleableOutputPin,
};

// Implementations for `Pin`
impl<const P: usize, const N: usize, MODE> ErrorType for Pin<P, N, MODE> {
    type Error = Infallible;
}

impl<const P: usize, const N: usize, MODE> OutputPin for Pin<P, N, Output<MODE>> {
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

impl<const P: usize, const N: usize, MODE> StatefulOutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|x| !x)
    }
}

impl<const P: usize, const N: usize, MODE> ToggleableOutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: usize, const N: usize, MODE> InputPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
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
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }
}

impl<MODE> ToggleableOutputPin for ErasedPin<Output<MODE>> {
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
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// Implementations for `PartiallyErasedPin`
impl<const P: usize, MODE> ErrorType for PartiallyErasedPin<P, MODE> {
    type Error = Infallible;
}

impl<const P: usize, MODE> OutputPin for PartiallyErasedPin<P, Output<MODE>> {
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

impl<const P: usize, MODE> StatefulOutputPin for PartiallyErasedPin<P, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }
}

impl<const P: usize, MODE> ToggleableOutputPin for PartiallyErasedPin<P, Output<MODE>> {
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: usize, MODE> InputPin for PartiallyErasedPin<P, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// Implementations for `DynamicPin
impl<const P: usize, const N: usize> ErrorType for DynamicPin<P, N> {
    type Error = PinModeError;
}

impl<const P: usize, const N: usize> OutputPin for DynamicPin<P, N> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }
}

impl<const P: usize, const N: usize> InputPin for DynamicPin<P, N> {
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}
