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
        if MODE::MODE == M::MODE {
            return;
        }

        change_pin_mode::<P, N, M>();
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> Pin<P, N, M> {
        self.mode::<M>();
        Pin::new()
    }
}

#[inline(always)]
fn change_pin_mode_port_pin<MODE: PinMode>(port: &crate::pac::port_00::Port00, pin: u8) {
    use crate::pac::*;
    let mode = MODE::MODE >> 3;

    unsafe {
        match pin {
            0 => port.iocr0().modify_atomic(|r| r.pc0().set(mode)),
            1 => port.iocr0().modify_atomic(|r| r.pc1().set(mode)),
            2 => port.iocr0().modify_atomic(|r| r.pc2().set(mode)),
            3 => port.iocr0().modify_atomic(|r| r.pc3().set(mode)),
            4 => port.iocr4().modify_atomic(|r| r.pc4().set(mode)),
            5 => port.iocr4().modify_atomic(|r| r.pc5().set(mode)),
            6 => port.iocr4().modify_atomic(|r| r.pc6().set(mode)),
            7 => port.iocr4().modify_atomic(|r| r.pc7().set(mode)),
            8 => port.iocr8().modify_atomic(|r| r.pc8().set(mode)),
            9 => port.iocr8().modify_atomic(|r| r.pc9().set(mode)),
            10 => port.iocr8().modify_atomic(|r| r.pc10().set(mode)),
            11 => port.iocr8().modify_atomic(|r| r.pc11().set(mode)),
            12 => port.iocr12().modify_atomic(|r| r.pc12().set(mode)),
            13 => port.iocr12().modify_atomic(|r| r.pc13().set(mode)),
            14 => port.iocr12().modify_atomic(|r| r.pc14().set(mode)),
            15 => port.iocr12().modify_atomic(|r| r.pc15().set(mode)),
            _ => unimplemented!(),
        }
    }
}

#[inline(always)]
fn change_pin_mode<const P: usize, const N: u8, M: PinMode>() {
    // SAFETY: All Port instances have the same layout as Port00
    change_pin_mode_port_pin::<M>(&unsafe { *Gpio::<P>::ptr() }, N)
}

use super::ErasedPin;
impl<MODE: PinMode> ErasedPin<MODE> {
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        let block = unsafe { self.block() };
        change_pin_mode_port_pin::<M>(block, self.pin_id());
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> ErasedPin<M> {
        self.mode::<M>();
        ErasedPin::new(self.port_id(), self.pin_id())
    }
}

use super::PartiallyErasedPin;
impl<const P: usize, MODE: PinMode> PartiallyErasedPin<P, MODE> {
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        let n = self.pin_id();
        change_pin_mode_port_pin::<M>(&unsafe { *Gpio::<P>::ptr() }, n)
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

/// Marker trait for valid pin modes (type state).
///
/// It can not be implemented by outside types.
pub trait PinMode: crate::Sealed {
    // TODO (alepez) check if MODE=FF is correct. I guess it should be the default value on the register.
    const MODE: u8 = 0xFF;
}

impl crate::Sealed for Input {}

impl PinMode for Input {
    const MODE: u8 = 0x00;
}

// TODO (alepez) Remove Analog
impl crate::Sealed for Analog {}

// TODO (alepez) Remove Analog
impl PinMode for Analog {
    const MODE: u8 = 0; // TODO (alepez)
}

impl<Otype> crate::Sealed for Output<Otype> {}

impl PinMode for Output<OpenDrain> {
    const MODE: u8 = 0xC0;
}

impl PinMode for Output<PushPull> {
    const MODE: u8 = 0x80;
}

impl<const A: u8, Otype> crate::Sealed for Alternate<A, Otype> {}

impl<const A: u8> PinMode for Alternate<A, OpenDrain> {
    const MODE: u8 = 0xC0 | (A << 3);
}

impl<const A: u8> PinMode for Alternate<A, PushPull> {
    const MODE: u8 = 0x80 | (A << 3);
}
