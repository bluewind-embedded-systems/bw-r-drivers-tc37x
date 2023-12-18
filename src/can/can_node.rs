use super::baud_rate::*;
use super::can_module::ClockSource;
use super::frame::Frame;
use super::CanModule;
use crate::util::wait_nop;

// TODO Default values are not valid
#[derive(Default)]
pub struct BaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
}

// TODO Default values are not valid
#[derive(Default)]
pub struct FastBaudRate {
    pub baud_rate: u32,
    pub sample_point: u16,
    pub sync_jump_with: u16,
    pub prescalar: u16,
    pub time_segment_1: u8,
    pub time_segment_2: u8,
    pub transceiver_delay_offset: u8,
}

#[derive(PartialEq, Debug, Default)]
pub enum FrameMode {
    // TODO refactor (annabo)
    #[default]
    Standard,
    FdLong,
    FdLongAndFast,
}
#[derive(PartialEq, Debug, Default)]
pub enum FrameType
// TODO refactor (annabo)
{
    #[default]
    Receive,
    Transmit,
    TransmitAndReceive,
    RemoteRequest,
    RemoteAnswer,
}

#[derive(Clone, Copy, Default)]
pub enum TxMode {
    #[default]
    DedicatedBuffers,
    Fifo,
    Queue,
    SharedFifo,
    SharedQueue,
}

#[derive(Clone, Copy, Default)]
pub enum RxMode {
    #[default]
    DedicatedBuffers,
    Fifo0,
    Fifo1,
    SharedFifo0,
    SharedFifo1,
    SharedAll,
}

#[derive(Default)]
pub struct CanNodeConfig {
    pub clock_source: ClockSource,
    pub calculate_bit_timing_values: bool,
    pub baud_rate: BaudRate,
    pub fast_baud_rate: FastBaudRate,
    pub frame_mode: FrameMode,
    pub frame_type: FrameType,
    pub tx_mode: TxMode,
    pub rx_mode: RxMode,
    pub tx_buffer_data_field_size: u8, //(TODO) limit possibile values to valid ones
    pub message_ram_tx_buffers_start_address: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId(pub(crate) u8);

impl NodeId {
    pub const fn new(n: u8) -> Self {
        Self(n)
    }
}

pub struct CanNode {
    module: CanModule,
    node_id: NodeId,
    inner: tc37x_pac::can0::Node,
}

impl CanNode {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: CanModule, node_id: NodeId) -> Self {
        let inner = module.registers().node(node_id.0.into());
        Self {
            module,
            node_id,
            inner,
        }
    }

    pub fn init(self, config: CanNodeConfig) -> Result<CanNode, ()> {
        self.module
            .set_clock_source(self.node_id.into(), config.clock_source);

        wait_nop(10);

        self.enable_configuration_change();

        self.configure_baud_rate(config.calculate_bit_timing_values, &config.baud_rate);

        // for CAN FD frames, set fast baud rate
        if config.frame_mode != FrameMode::Standard {
            self.configure_fast_baud_rate(
                config.calculate_bit_timing_values,
                &config.fast_baud_rate,
            );
        }

        // transmit frame configuration
        if let FrameType::Transmit
        | FrameType::TransmitAndReceive
        | FrameType::RemoteRequest
        | FrameType::RemoteAnswer = config.frame_type
        {
            self.set_tx_buffer_data_field_size(config.tx_buffer_data_field_size);
            self.set_tx_buffer_start_address(config.message_ram_tx_buffers_start_address);
        }

        self.disable_configuration_change();

        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }

    #[inline]
    fn enable_configuration_change(&self) {
        let cccr = self.inner.cccr();

        if unsafe { cccr.read() }.init().get() {
            unsafe { cccr.modify(|r| r.cce().set(false)) };
            while unsafe { cccr.read() }.cce().get() {}

            unsafe { cccr.modify(|r| r.init().set(false)) };
            while unsafe { cccr.read() }.init().get() {}
        }

        unsafe { cccr.modify(|r| r.init().set(true)) };
        while !unsafe { cccr.read() }.init().get() {}

        unsafe { cccr.modify(|r| r.cce().set(true).init().set(true)) };
    }

    #[inline]
    pub fn disable_configuration_change(&self) {
        let cccr = self.inner.cccr();

        unsafe { cccr.modify(|r| r.cce().set(false)) };

        while unsafe { cccr.read() }.cce().get() {}

        unsafe { cccr.modify(|r| r.init().set(false)) };

        while unsafe { cccr.read() }.init().get() {}
    }

    fn configure_baud_rate(&self, calculate_bit_timing_values: bool, baud_rate: &BaudRate) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            let timing: BitTiming = calculate_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
            self.set_bit_timing(timing);
        } else {
            self.set_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar,
            )
        }
    }

    fn configure_fast_baud_rate(
        &self,
        calculate_bit_timing_values: bool,
        baud_rate: &FastBaudRate,
    ) {
        if calculate_bit_timing_values {
            let module_freq = crate::scu::ccu::get_mcan_frequency() as f32;
            self.set_fast_bit_timing(
                module_freq,
                baud_rate.baud_rate,
                baud_rate.sample_point,
                baud_rate.sync_jump_with,
            );
        } else {
            self.set_fast_bit_timing_values(
                baud_rate.sync_jump_with as u8,
                baud_rate.time_segment_2,
                baud_rate.time_segment_1,
                baud_rate.prescalar as u8,
            );
        }

        if baud_rate.transceiver_delay_offset != 0 {
            self.set_transceiver_delay_compensation_offset(baud_rate.transceiver_delay_offset);
        }
    }

    fn set_bit_timing(&self, timing: BitTiming) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nbrp()
                    .set(timing.brp)
                    .nsjw()
                    .set(timing.sjw)
                    .ntseg1()
                    .set(timing.tseg1)
                    .ntseg2()
                    .set(timing.tseg2)
            })
        }
    }

    fn set_bit_timing_values(&self, sjw: u8, time_segment2: u8, time_segment1: u8, prescaler: u16) {
        unsafe {
            self.inner.nbtp().modify(|r| {
                r.nsjw()
                    .set(sjw)
                    .ntseg1()
                    .set(time_segment1)
                    .ntseg2()
                    .set(time_segment2)
                    .nbrp()
                    .set(prescaler)
            })
        };
    }

    fn set_fast_bit_timing(&self, module_freq: f32, baudrate: u32, sample_point: u16, sjw: u16) {
        let timing = calculate_fast_bit_timing(module_freq, baudrate, sample_point, sjw);

        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dbrp()
                    .set(timing.brp.try_into().unwrap())
                    .dsjw()
                    .set(timing.sjw)
                    .dtseg1()
                    .set(timing.tseg1)
                    .dtseg2()
                    .set(timing.tseg2)
            })
        }
    }

    pub fn set_fast_bit_timing_values(
        &self,
        sjw: u8,
        time_segment2: u8,
        time_segment1: u8,
        prescaler: u8,
    ) {
        unsafe {
            self.inner.dbtp().modify(|r| {
                r.dsjw()
                    .set(sjw)
                    .dtseg1()
                    .set(time_segment1)
                    .dtseg2()
                    .set(time_segment2)
                    .dbrp()
                    .set(prescaler)
            })
        };
    }

    pub fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
        unsafe { self.inner.dbtp().modify(|r| r.tdc().set(true)) };
        unsafe { self.inner.tdcr().modify(|r| r.tdco().set(delay)) };
    }
}

// IfxLld_Can_Std_Rx_Element_Functions
impl CanNode {
    fn get_rx_fifo0_fill_level(&self) -> u8 {
        unsafe { self.inner.rxf0s().read() }.f0fl().get()
    }

    fn get_rx_fifo1_fill_level(&self) -> u8 {
        unsafe { self.inner.rxf1s().read() }.f1fl().get()
    }

    #[inline]
    fn set_rx_buffers_start_address(&self, address: u16) {
        unsafe { self.inner.rxbc().modify(|r| r.rbsa().set(address >> 2)) };
    }

    #[inline]
    fn set_rx_fifo0_size(&self, size: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0s().set(size)) };
    }

    #[inline]
    fn set_rx_fifo0_start_address(&self, address: u16) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0sa().set(address >> 2)) };
    }

    #[inline]
    fn set_rx_fifo0_watermark_level(&self, level: u8) {
        unsafe { self.inner.rxf0c().modify(|r| r.f0wm().set(level)) };
    }

    #[inline]
    fn set_rx_fifo1_size(&self, size: u8) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1s().set(size)) };
    }

    #[inline]
    fn set_rx_fifo1_start_address(&self, address: u16) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1sa().set(address >> 2)) };
    }

    #[inline]
    fn set_rx_fifo1_watermark_level(&self, level: u8) {
        unsafe { self.inner.rxf1c().modify(|r| r.f1wm().set(level)) };
    }
}

impl CanNode {
    #[inline]
    fn is_tx_event_fifo_element_lost(&self) -> bool {
        unsafe { self.inner.txefs().read() }.tefl().get()
    }

    #[inline]
    fn is_tx_event_fifo_full(&self) -> bool {
        unsafe { self.inner.txefs().read() }.eff().get()
    }

    #[inline]
    fn is_tx_fifo_queue_full(&self) -> bool {
        unsafe { self.inner.txfqs().read() }.tfqf().get()
    }

    #[inline]
    fn pause_trasmission(&self, enable: bool) {
        unsafe { self.inner.cccr().modify(|r| r.txp().set(enable)) };
    }

    #[inline]
    fn set_dedicated_tx_buffers_number(&self, number: u8) {
        unsafe { self.inner.txbc().modify(|r| r.ndtb().set(number)) };
    }

    fn set_tx_buffer_data_field_size(&self, data_field_size: u8) {
        let data_field_size = tc37x_pac::can0::node::txesc::Tbds(data_field_size);
        unsafe { self.inner.txesc().modify(|r| r.tbds().set(data_field_size)) };
    }

    #[inline]
    fn set_tx_buffer_start_address(&self, address: u16) {
        unsafe { self.inner.txbc().modify(|r| r.tbsa().set(address >> 2)) };
    }

    fn set_transmit_fifo_queue_size(&self, number: u8) {
        unsafe { self.inner.txbc().modify(|r| r.tfqs().set(number)) };
    }
}

impl CanNode {
    #[inline]
    fn set_tx_event_fifo_start_address(&self, address: u16) {
        unsafe { self.inner.txefc().modify(|r| r.efsa().set(address >> 2)) };
    }

    #[inline]
    fn set_tx_event_fifo_size(&self, size: u8) {
        unsafe { self.inner.txefc().modify(|r| r.efs().set(size)) };
    }
}

impl CanNode {
    #[inline]
    fn set_standard_filter_list_start_address(&self, address: u16) {
        unsafe { self.inner.sidfc().modify(|r| r.flssa().set(address >> 2)) };
    }

    #[inline]
    fn set_standard_filter_list_size(&self, size: u8) {
        unsafe { self.inner.sidfc().modify(|r| r.lss().set(size)) };
    }

    #[inline]
    fn reject_remote_frames_with_standard_id(&self) {
        unsafe { self.inner.gfc().modify(|r| r.rrfs().set(true)) };
    }

    #[inline]
    fn set_extended_filter_list_start_address(&self, address: u16) {
        unsafe { self.inner.xidfc().modify(|r| r.flesa().set(address >> 2)) };
    }

    #[inline]
    pub fn set_extended_filter_list_size(&self, size: u8) {
        unsafe { self.inner.xidfc().modify(|r| r.lse().set(size)) };
    }

    fn reject_remote_frames_with_extended_id(&self) {
        unsafe { self.inner.gfc().modify(|r| r.rrfe().set(true)) };
    }
}
