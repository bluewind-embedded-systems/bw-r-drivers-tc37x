use super::*;

impl<const P: usize, const N: u8, const A: u8> Pin<P, N, Alternate<A, PushPull>> {
    /// Turns pin alternate configuration pin into open drain
    pub fn set_open_drain(self) -> Pin<P, N, Alternate<A, OpenDrain>> {
        self.into_mode()
    }
}

impl<const P: usize, const N: u8, MODE: PinMode> Pin<P, N, MODE> {
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(self) -> Pin<P, N, Alternate<A, PushPull>>
    where
        Self: marker::IntoAf<A>,
    {
        self.into_mode()
    }

    /// Configures the pin to operate in alternate open drain mode
    #[allow(path_statements)]
    pub fn into_alternate_open_drain<const A: u8>(self) -> Pin<P, N, Alternate<A, OpenDrain>>
    where
        Self: marker::IntoAf<A>,
    {
        self.into_mode()
    }

    /// Configures the pin to operate as a input pin
    pub fn into_input(self) -> Pin<P, N, Input> {
        self.into_mode()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::None)
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::Down)
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::Up)
    }

    /// Configures the pin to operate as an open drain output pin
    /// Initial state will be low.
    pub fn into_open_drain_output(self) -> Pin<P, N, Output<OpenDrain>> {
        self.into_mode()
    }

    /// Configures the pin to operate as an open-drain output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_open_drain_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<OpenDrain>> {
        self._set_state(initial_state);
        self.into_mode()
    }

    /// Configures the pin to operate as an push pull output pin
    /// Initial state will be low.
    pub fn into_push_pull_output(mut self) -> Pin<P, N, Output<PushPull>> {
        self._set_low();
        self.into_mode()
    }

    /// Configures the pin to operate as an push-pull output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_push_pull_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<PushPull>> {
        self._set_state(initial_state);
        self.into_mode()
    }

    /// Configures the pin to operate as an analog input pin
    pub fn into_analog(self) -> Pin<P, N, Analog> {
        self.into_mode()
    }

    /// Configures the pin as a pin that can change between input
    /// and output without changing the type. It starts out
    /// as a floating input
    pub fn into_dynamic(self) -> DynamicPin<P, N> {
        self.into_floating_input();
        DynamicPin::new(Dynamic::InputFloating)
    }

    /// Puts `self` into mode `M`.
    ///
    /// This violates the type state constraints from `MODE`, so callers must
    /// ensure they use this properly.
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        change_mode!((*Gpio::<P>::ptr()), N);
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> Pin<P, N, M> {
        self.mode::<M>();
        Pin::new()
    }
}
macro_rules! change_mode {
    ($block:expr, $N:ident) => {
        use tc37x_pac::hidden::RegValue;
        use crate::pac;

        let shift : usize = (($N & 0x3) * 8).into();
        let iocr_offset : usize = ($N / 4).into();

        // FIXME (alepez) this is always OUTPUT_PUSH_PULL_GENERAL, must be converted from MODE
        let mode = 0x80;

        let mode_bits : u32 = (mode) << shift;
        let mode_mask : u32 = 0xFF << shift;

        // Violates pac APIs, but it's a simple way to select the correct IOCR register, given
        // the port and the pin index.
        let iocr: pac::Reg<pac::port_00::Iocr0, pac::RW> = unsafe {
            let iocr0 = $block.iocr0();
            let addr: *mut u32 = core::mem::transmute(iocr0);
            let addr = addr.add(iocr_offset);
            core::mem::transmute(addr)
        };

        unsafe {
            iocr.modify_atomic(|mut r| {
                *r.data_mut_ref() = mode_bits;
                *r.get_mask_mut_ref() = mode_mask;
                r
            })
        };
    };
}

use change_mode;

use super::ErasedPin;
impl<MODE: PinMode> ErasedPin<MODE> {
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        let n = self.pin_id();
        change_mode!(self.block(), n);
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> ErasedPin<M> {
        self.mode::<M>();
        ErasedPin::from_pin_port(self.into_pin_port())
    }
}

use super::PartiallyErasedPin;
impl<const P: usize, MODE: PinMode> PartiallyErasedPin<P, MODE> {
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        let n = self.pin_id();
        change_mode!((*Gpio::<P>::ptr()), n);
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> PartiallyErasedPin<P, M> {
        self.mode::<M>();
        PartiallyErasedPin::new(self.i)
    }
}

impl<const P: usize, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: PinMode,
{
    fn with_mode<M, F, R>(&mut self, f: F) -> R
    where
        M: PinMode,
        F: FnOnce(&mut Pin<P, N, M>) -> R,
    {
        self.mode::<M>(); // change physical mode, without changing typestate

        // This will reset the pin back to the original mode when dropped.
        // (so either when `with_mode` returns or when `f` unwinds)
        let mut resetti = ResetMode::<P, N, M, MODE>::new();

        f(&mut resetti.pin)
    }

    /// Temporarily configures this pin as a input.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_input<R>(&mut self, f: impl FnOnce(&mut Pin<P, N, Input>) -> R) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an analog pin.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    pub fn with_analog<R>(&mut self, f: impl FnOnce(&mut Pin<P, N, Analog>) -> R) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an open drain output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// The value of the pin after conversion is undefined. If you
    /// want to control it, use `with_open_drain_output_in_state`
    pub fn with_open_drain_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Output<OpenDrain>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as an open drain output .
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// Note that the new state is set slightly before conversion
    /// happens. This can cause a short output glitch if switching
    /// between output modes
    pub fn with_open_drain_output_in_state<R>(
        &mut self,
        state: PinState,
        f: impl FnOnce(&mut Pin<P, N, Output<OpenDrain>>) -> R,
    ) -> R {
        self._set_state(state);
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a push-pull output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// The value of the pin after conversion is undefined. If you
    /// want to control it, use `with_push_pull_output_in_state`
    pub fn with_push_pull_output<R>(
        &mut self,
        f: impl FnOnce(&mut Pin<P, N, Output<PushPull>>) -> R,
    ) -> R {
        self.with_mode(f)
    }

    /// Temporarily configures this pin as a push-pull output.
    ///
    /// The closure `f` is called with the reconfigured pin. After it returns,
    /// the pin will be configured back.
    /// Note that the new state is set slightly before conversion
    /// happens. This can cause a short output glitch if switching
    /// between output modes
    pub fn with_push_pull_output_in_state<R>(
        &mut self,
        state: PinState,
        f: impl FnOnce(&mut Pin<P, N, Output<PushPull>>) -> R,
    ) -> R {
        self._set_state(state);
        self.with_mode(f)
    }
}

/// Wrapper around a pin that transitions the pin to mode ORIG when dropped
struct ResetMode<const P: usize, const N: u8, CURRENT: PinMode, ORIG: PinMode> {
    pub pin: Pin<P, N, CURRENT>,
    _mode: PhantomData<ORIG>,
}
impl<const P: usize, const N: u8, CURRENT: PinMode, ORIG: PinMode> ResetMode<P, N, CURRENT, ORIG> {
    fn new() -> Self {
        Self {
            pin: Pin::new(),
            _mode: PhantomData,
        }
    }
}
impl<const P: usize, const N: u8, CURRENT: PinMode, ORIG: PinMode> Drop
    for ResetMode<P, N, CURRENT, ORIG>
{
    fn drop(&mut self) {
        self.pin.mode::<ORIG>();
    }
}

// TODO (alepez) Change PinMode to work wit TC37 modes:
//     pub const INPUT_NO_PULL_DEVICE: Mode = Self(0);
//     pub const INPUT_PULL_DOWN: Mode = Self(8);
//     pub const INPUT_PULL_UP: Mode = Self(0x10);
//     pub const OUTPUT_PUSH_PULL_GENERAL: Mode = Self(0x80);
//     pub const OUTPUT_PUSH_PULL_ALT1: Mode = Self(0x88);
//     pub const OUTPUT_PUSH_PULL_ALT2: Mode = Self(0x90);
//     pub const OUTPUT_PUSH_PULL_ALT3: Mode = Self(0x98);
//     pub const OUTPUT_PUSH_PULL_ALT4: Mode = Self(0xA0);
//     pub const OUTPUT_PUSH_PULL_ALT5: Mode = Self(0xA8);
//     pub const OUTPUT_PUSH_PULL_ALT6: Mode = Self(0xB0);
//     pub const OUTPUT_PUSH_PULL_ALT7: Mode = Self(0xB8);
//     pub const OUTPUT_OPEN_DRAIN_GENERAL: Mode = Self(0xC0);
//     pub const OUTPUT_OPEN_DRAIN_ALT1: Mode = Self(0xC8);
//     pub const OUTPUT_OPEN_DRAIN_ALT2: Mode = Self(0xD0);
//     pub const OUTPUT_OPEN_DRAIN_ALT3: Mode = Self(0xD8);
//     pub const OUTPUT_OPEN_DRAIN_ALT4: Mode = Self(0xE0);
//     pub const OUTPUT_OPEN_DRAIN_ALT5: Mode = Self(0xE8);
//     pub const OUTPUT_OPEN_DRAIN_ALT6: Mode = Self(0xF0);
//     pub const OUTPUT_OPEN_DRAIN_ALT7: Mode = Self(0xF8);

/// Marker trait for valid pin modes (type state).
///
/// It can not be implemented by outside types.
pub trait PinMode: crate::Sealed {
    // These constants are used to implement the pin configuration code.
    // They are not part of public API.

    #[doc(hidden)]
    const MODER: u32 = u32::MAX;
    #[doc(hidden)]
    const OTYPER: Option<u32> = None;
    #[doc(hidden)]
    const AFR: Option<u32> = None;
}

impl crate::Sealed for Input {}
impl PinMode for Input {
    const MODER: u32 = 0b00;
}

impl crate::Sealed for Analog {}
impl PinMode for Analog {
    const MODER: u32 = 0b11;
}

impl<Otype> crate::Sealed for Output<Otype> {}
impl PinMode for Output<OpenDrain> {
    const MODER: u32 = 0b01;
    const OTYPER: Option<u32> = Some(0b1);
}

impl PinMode for Output<PushPull> {
    const MODER: u32 = 0b01;
    const OTYPER: Option<u32> = Some(0b0);
}

impl<const A: u8, Otype> crate::Sealed for Alternate<A, Otype> {}
impl<const A: u8> PinMode for Alternate<A, OpenDrain> {
    const MODER: u32 = 0b10;
    const OTYPER: Option<u32> = Some(0b1);
    const AFR: Option<u32> = Some(A as _);
}

impl<const A: u8> PinMode for Alternate<A, PushPull> {
    const MODER: u32 = 0b10;
    const OTYPER: Option<u32> = Some(0b0);
    const AFR: Option<u32> = Some(A as _);
}
