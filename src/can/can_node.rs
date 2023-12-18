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
}

impl CanNode {
    /// Only a module can create a node. This function is only accessible from within this crate.
    pub(crate) fn new(module: CanModule, node_id: NodeId) -> Self {
        //let inner = module.registers().node(node_id.0.into());
        Self { module, node_id }
    }

    pub fn init(self, config: CanNodeConfig) -> Result<CanNode, ()> {
        self.module
            .set_clock_source(self.node_id.into(), config.clock_source);

        wait_nop(10);

        self.enable_configuration_change();

        self.configure_baud_rate(config.calculate_bit_timing_values, &config.baud_rate);

        // for CAN FD frames, set fast baudrate
        if config.frame_mode != FrameMode::Standard {
            self.configure_fast_baud_rate(
                config.calculate_bit_timing_values,
                &config.fast_baud_rate,
            );
        }

        /* transmit frame configuration */
        if let FrameType::Transmit
        | FrameType::TransmitAndReceive
        | FrameType::RemoteRequest
        | FrameType::RemoteAnswer = config.frame_type
        {
            #[cfg(feature = "log")]
            defmt::debug!("transmit frame type configuration");
            self.set_tx_buffer_data_field_size(config.tx_buffer_data_field_size);
            self.set_tx_buffer_start_address(config.message_ram_tx_buffers_start_address);

            let mode = config.tx_mode;
            // match mode {
            //     TxMode::DedicatedBuffers | TxMode::SharedFifo | TxMode::SharedQueue => {
            //         set_dedicated_tx_buffers_number(
            //             self.tx_config.dedicated_tx_buffers_number,
            //         );
            //         if let TxMode::SharedFifo | TxMode::SharedQueue = mode {
            //             if let TxMode::SharedFifo = mode {
            //                 set_transmit_fifo_queue_mode(TxMode::Fifo);
            //             }
            //             if let TxMode::SharedQueue = mode {
            //                 set_transmit_fifo_queue_mode(TxMode::Queue);
            //             }
            //             set_transmit_fifo_queue_size(self.tx_config.fifo_queue_size);
            //         }
            //         for id in 0..self.tx_config.dedicated_tx_buffers_number
            //             + self.tx_config.fifo_queue_size
            //         {
            //             enable_tx_buffer_transmission_interrupt(TxBufferId(id));
            //         }
            //     }
            //     TxMode::Fifo | TxMode::Queue => {
            //         set_transmit_fifo_queue_mode(mode);
            //         set_transmit_fifo_queue_size(self.tx_config.fifo_queue_size);
            //         for id in 0..self.tx_config.fifo_queue_size {
            //             enable_tx_buffer_transmission_interrupt(TxBufferId(id));
            //         }
            //     }
            // }

            // if (1..=32).contains(&self.tx_config.event_fifo_size) {
            //     set_tx_event_fifo_start_address(self.message_ram.tx_event_fifo_start_address);
            //     set_tx_event_fifo_size(self.tx_config.event_fifo_size);
            // } else {
            //     #[cfg(feature = "log")]
            //     defmt::assert!(self.tx_config.event_fifo_size <= 32)
            // }

            // set_frame_mode(self.frame.mode);
        }

        /* recieve frame configuration */
        //if(){}

        // if(config.pins ... )
        // {

        // }

        // if(config.loopback_enabled){
        //     ...
        // }

        // interrupt groups configuration
        //{

        //}
        self.disable_configuration_change();

        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }

    #[inline]
    fn enable_configuration_change(&self) {
        let cccr = tc37x_pac::CAN0.cccr0();

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
        let cccr = tc37x_pac::CAN0.cccr0();

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
            tc37x_pac::CAN0.nbtp0().modify(|r| {
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
            tc37x_pac::CAN0.nbtp0().modify(|r| {
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
            tc37x_pac::CAN0.dbtp0().modify(|r| {
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
            tc37x_pac::CAN0.dbtp0().modify(|r| {
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
        unsafe { tc37x_pac::CAN0.dbtp0().modify(|r| r.tdc().set(true)) };
        unsafe { tc37x_pac::CAN0.tdcr0().modify(|r| r.tdco().set(delay)) };
    }
}

// IfxLld_Can_Std_Rx_Element_Functions
impl CanNode {
    fn get_rx_fifo0_fill_level(&self) -> u8 {
        unsafe { tc37x_pac::CAN0.rxf0s0().read() }.f0fl().get()
    }

    // fn get_rx_fifo0_get_index(&self) -> RxBufferId {
    //     let id = unsafe { tc37x_pac::CAN0.rxf0s0().read() }.f0gi().get();
    //     RxBufferId::new_const(id)
    // }

    fn get_rx_fifo1_fill_level(&self) -> u8 {
        unsafe { tc37x_pac::CAN0.rxf1s0().read() }.f1fl().get()
    }

    // fn get_rx_fifo1_get_index(&self) -> RxBufferId {
    //     let id = unsafe { tc37x_pac::CAN0.rxf1s0().read() }.f1gi().get();
    //     RxBufferId::new_const(id)
    // }

    // #[inline]
    // pub fn set_rx_buffer_data_field_size(&self, size: DataFieldSize) {
    //     unsafe { tc37x_pac::CAN0.rxesc0().modify(|r| r.rbds().set(size.into())) };
    // }

    //  fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: RxBufferId) -> bool {
    //     let (data, mask) = if rx_buffer_id < RxBufferId::new_const(32) {
    //         let data = unsafe { tc37x_pac::CAN0.ndat10().read() }.data();
    //         let mask = 1 << u8::from(rx_buffer_id);
    //         (data, mask)
    //     } else {
    //         let data = unsafe { tc37x_pac::CAN0.ndat20().read() }.data();
    //         let mask = 1 << (u8::from(rx_buffer_id) - 32);
    //         (data, mask)
    //     };
    //     (data & mask) != 0
    // }

    // #[inline]
    // fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxf0a0()
    //             .modify(|r| r.f0ai().set(rx_buffer_id.into()))
    //     };
    // }

    // #[inline]
    // fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxf1a0()
    //             .modify(|r| r.f1ai().set(rx_buffer_id.into()))
    //     };
    // }

    // fn clear_rx_buffer_new_data_flag(&self, rx_buffer_id: RxBufferId) {
    //     if rx_buffer_id < RxBufferId::new_const(32) {
    //         unsafe {
    //             tc37x_pac::CAN0.ndat10().init(|mut r| {
    //                 *r.data_mut_ref() = 1u32 << u8::from(rx_buffer_id);
    //                 r
    //             })
    //         };
    //     } else {
    //         unsafe {
    //             tc37x_pac::CAN0.ndat20().init(|mut r| {
    //                 *r.data_mut_ref() = 1u32 << (u8::from(rx_buffer_id) - 32);
    //                 r
    //             })
    //         };
    //     }
    // }

    #[inline]
    fn set_rx_buffers_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .rxbc0()
                .modify(|r| r.rbsa().set(address >> 2))
        };
    }

    // #[inline]
    // fn set_rx_fifo0_data_field_size(&self, size: DataFieldSize) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxesc0()
    //             .modify(|r| r.f0ds().set(can0::node::rxesc::F0Ds(size.into())))
    //     };
    // }

    // #[inline]
    // fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxf0c0()
    //             .modify(|r| r.f0om().set(mode == RxFifoMode::Overwrite))
    //     };
    // }

    #[inline]
    fn set_rx_fifo0_size(&self, size: u8) {
        unsafe { tc37x_pac::CAN0.rxf0c0().modify(|r| r.f0s().set(size)) };
    }

    #[inline]
    fn set_rx_fifo0_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .rxf0c0()
                .modify(|r| r.f0sa().set(address >> 2))
        };
    }

    #[inline]
    fn set_rx_fifo0_watermark_level(&self, level: u8) {
        unsafe { tc37x_pac::CAN0.rxf0c0().modify(|r| r.f0wm().set(level)) };
    }

    // #[inline]
    // fn set_rx_fifo1_data_field_size(&self, size: DataFieldSize) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxesc0()
    //             .modify(|r| r.f1ds().set(can0::node::rxesc::F1Ds(size.into())))
    //     };
    // }

    // #[inline]
    // fn set_rx_fifo1_operating_mode(&self, mode: RxFifoMode) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .rxf1c0()
    //             .modify(|r| r.f1om().set(mode == RxFifoMode::Overwrite))
    //     };
    // }

    #[inline]
    fn set_rx_fifo1_size(&self, size: u8) {
        unsafe { tc37x_pac::CAN0.rxf1c0().modify(|r| r.f1s().set(size)) };
    }

    #[inline]
    fn set_rx_fifo1_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .rxf1c0()
                .modify(|r| r.f1sa().set(address >> 2))
        };
    }

    #[inline]
    fn set_rx_fifo1_watermark_level(&self, level: u8) {
        unsafe { tc37x_pac::CAN0.rxf1c0().modify(|r| r.f1wm().set(level)) };
    }
}

// IfxLld_Can_Std_Tx_Element_Functions
impl CanNode {
    // #[inline]
    // fn get_tx_fifo_queue_put_index(&self) -> TxBufferId {
    //     let id = unsafe { tc37x_pac::CAN0.txfqs().read() }.tfqpi().get();
    //     TxBufferId::new_const(id)
    // }

    // #[inline]
    // fn is_tx_buffer_cancellation_finished(&self, tx_buffer_id: TxBufferId) -> bool {
    //     self.is_tx_buffer_transmission_occured(tx_buffer_id)
    // }

    // #[inline]
    // fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
    //     unsafe { tc37x_pac::CAN0.txbrp0().read() }
    //         .trp(tx_buffer_id.into())
    //         .get()
    // }

    // #[inline]
    // fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: TxBufferId) -> bool {
    //     let data = unsafe { tc37x_pac::CAN0.txbto0().read() }.data();
    //     let mask = 1u32 << u32::from(tx_buffer_id);
    //     (data & mask) != 0
    // }

    #[inline]
    fn is_tx_event_fifo_element_lost(&self) -> bool {
        unsafe { tc37x_pac::CAN0.txefs0().read() }.tefl().get()
    }

    #[inline]
    fn is_tx_event_fifo_full(&self) -> bool {
        unsafe { tc37x_pac::CAN0.txefs0().read() }.eff().get()
    }

    #[inline]
    fn is_tx_fifo_queue_full(&self) -> bool {
        unsafe { tc37x_pac::CAN0.txfqs0().read() }.tfqf().get()
    }

    #[inline]
    fn pause_trasmission(&self, enable: bool) {
        unsafe { tc37x_pac::CAN0.cccr0().modify(|r| r.txp().set(enable)) };
    }

    #[inline]
    fn set_dedicated_tx_buffers_number(&self, number: u8) {
        unsafe { tc37x_pac::CAN0.txbc0().modify(|r| r.ndtb().set(number)) };
    }

    #[inline]
    // fn set_tx_buffer_add_request(&self, tx_buffer_id: TxBufferId) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .txbar0()
    //             .modify(|r| r.ar(tx_buffer_id.into()).set(true))
    //     }
    // }
    #[inline]
    fn set_tx_buffer_data_field_size(&self, data_field_size: u8) {
        unsafe {
            tc37x_pac::CAN0
                .txesc0()
                .modify(|r| r.tbds().set(data_field_size))
        };
    }

    #[inline]
    fn set_tx_buffer_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .txbc0()
                .modify(|r| r.tbsa().set(address >> 2))
        };
    }

    #[inline]
    // pub fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
    //     if let TxMode::Fifo | TxMode::Queue = mode {
    //         let val = (mode as u8) != 0;
    //         unsafe { tc37x_pac::CAN0.txbc0().modify(|r| r.tfqm().set(val)) };
    //     } else {
    //         panic!("invalid fifo queue mode");
    //     }
    // }
    #[inline]
    fn set_transmit_fifo_queue_size(&self, number: u8) {
        unsafe { tc37x_pac::CAN0.txbc0().modify(|r| r.tfqs().set(number)) };
    }

    // fn get_data_field_size(&self, from: ReadFrom) -> u8 {
    //     let rx_esc = unsafe { tc37x_pac::CAN0.rxesc0().read() };
    //     let size_code = match from {
    //         ReadFrom::Buffer(_) => rx_esc.rbds().get().0,
    //         ReadFrom::RxFifo0 => rx_esc.f0ds().get().0,
    //         ReadFrom::RxFifo1 => rx_esc.f1ds().get().0,
    //     };

    //     if size_code < DataFieldSize::_32.into() {
    //         (size_code + 2) * 4
    //     } else {
    //         (size_code - 3) * 16
    //     }
    // }

    // fn get_tx_buffer_data_field_size(&self) -> u8 {
    //     let size_code = unsafe { tc37x_pac::CAN0.txesc0().read() }.tbds().get();

    //     if size_code < DataFieldSize::_32.into() {
    //         (size_code + 2) * 4
    //     } else {
    //         (size_code - 3) * 16
    //     }
    // }

    // fn get_rx_element_address(
    //     &self,
    //     ram_base_address: u32,
    //     tx_buffers_start_address: u16,
    //     buf_from: ReadFrom,
    //     buffer_number: RxBufferId,
    // ) -> Rx {
    //     let num_of_config_bytes = 8u32;
    //     let num_of_data_bytes = self.get_data_field_size(buf_from) as u32;
    //     let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
    //     let tx_buffer_index = tx_buffer_size * u32::from(buffer_number);

    //     let tx_buffer_element_address =
    //         ram_base_address + tx_buffers_start_address as u32 + tx_buffer_index;

    //     Rx::new(tx_buffer_element_address as *mut u8)
    // }

    // fn get_tx_element_address(
    //     &self,
    //     ram_base_address: u32,
    //     tx_buffers_start_address: u16,
    //     buffer_number: TxBufferId,
    // ) -> Tx {
    //     let num_of_config_bytes = 8u32;
    //     let num_of_data_bytes = self.get_tx_buffer_data_field_size() as u32;
    //     let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
    //     let tx_buffer_index = tx_buffer_size * u32::from(buffer_number);

    //     let tx_buffer_element_address =
    //         ram_base_address + tx_buffers_start_address as u32 + tx_buffer_index;

    //     Tx::new(tx_buffer_element_address as *mut u8)
    // }
}

// IfxLld_Can_Std_Tx_Event_FIFO_Element_Functions
impl CanNode {
    #[inline]
    fn set_tx_event_fifo_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .txefc0()
                .modify(|r| r.efsa().set(address >> 2))
        };
    }

    #[inline]
    fn set_tx_event_fifo_size(&self, size: u8) {
        unsafe { tc37x_pac::CAN0.txefc0().modify(|r| r.efs().set(size)) };
    }
}

// IfxLld_Can_Std_Interrupt_Functions
impl CanNode {
    // #[inline]
    // fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
    //     unsafe {
    //         tc37x_pac::CAN0.txbtie0().modify(|mut r| {
    //             *r.data_mut_ref() |= 1 << tx_buffer_id.0;
    //             r
    //         })
    //     };
    // }

    // fn set_group_interrupt_line(
    //     &self,
    //     interrupt_group: InterruptGroup,
    //     interrupt_line: InterruptLine,
    // ) {
    //     if interrupt_group <= InterruptGroup::Loi {
    //         unsafe {
    //             tc37x_pac::CAN0.grint10().modify(|mut r| {
    //                 *r.data_mut_ref() |= (interrupt_line.0 as u32) << (interrupt_group as u32 * 4);
    //                 r
    //             })
    //         };
    //     } else {
    //         unsafe {
    //             tc37x_pac::CAN0.grint20().modify(|mut r| {
    //                 *r.data_mut_ref() |=
    //                     (interrupt_line.0 as u32) << ((interrupt_group as u32 % 8) * 4);
    //                 r
    //             })
    //         };
    //     }
    // }

    // #[inline]
    // fn enable_interrupt(&self, interrupt: Interrupt) {
    //     unsafe {
    //         tc37x_pac::CAN0.ie0().modify(|mut r| {
    //             *r.data_mut_ref() |= 1 << interrupt as u32;
    //             r
    //         })
    //     };
    // }

    // #[inline]
    // fn clear_interrupt_flag(&self, interrupt: Interrupt) {
    //     unsafe {
    //         tc37x_pac::CAN0.ir0().init(|mut r| {
    //             *r.data_mut_ref() = 1 << interrupt as u32;
    //             r
    //         })
    //     };
    // }
}

// IfxLld_Can_Std_Filter_Functions
impl CanNode {
    #[inline]
    fn set_standard_filter_list_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .sidfc0()
                .modify(|r| r.flssa().set(address >> 2))
        };
    }

    #[inline]
    fn set_standard_filter_list_size(&self, size: u8) {
        unsafe { tc37x_pac::CAN0.sidfc0().modify(|r| r.lss().set(size)) };
    }

    //#[inline]
    // fn configure_standard_filter_for_non_matching_frame(&self, filter: NonMatchingFrame) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .gfc0()
    //             .modify(|r| r.anfs().set(can0::node::gfc::Anfs(filter as _)))
    //     };
    // }

    #[inline]
    fn reject_remote_frames_with_standard_id(&self) {
        unsafe { tc37x_pac::CAN0.gfc0().modify(|r| r.rrfs().set(true)) };
    }

    #[inline]
    fn set_extended_filter_list_start_address(&self, address: u16) {
        unsafe {
            tc37x_pac::CAN0
                .xidfc0()
                .modify(|r| r.flesa().set(address >> 2))
        };
    }

    #[inline]
    pub fn set_extended_filter_list_size(&self, size: u8) {
        unsafe { tc37x_pac::CAN0.xidfc0().modify(|r| r.lse().set(size)) };
    }

    #[inline]
    // fn configure_extended_filter_for_non_matching_frame(&self, filter: NonMatchingFrame) {
    //     unsafe {
    //         tc37x_pac::CAN0
    //             .gfc0()
    //             .modify(|r| r.anfe().set(can0::node::gfc::Anfe(filter as _)))
    //     };
    // }
    #[inline]
    fn reject_remote_frames_with_extended_id(&self) {
        unsafe { tc37x_pac::CAN0.gfc0().modify(|r| r.rrfe().set(true)) };
    }
}

// fn get_standard_filter_element_address(
//     ram_base_address: u32,
//     standard_filter_list_start_address: u16,
//     filter_number: u8,
// ) -> StdMsg {
//     let filter_index = filter_number as u32 * 4;
//     let standard_filter_element_address =
//         ram_base_address + standard_filter_list_start_address as u32 + filter_index;
//     StdMsg::new(standard_filter_element_address as _)
// }

// pub fn get_extended_filter_element_address(
//     ram_base_address: u32,
//     extended_filter_list_start_address: u16,
//     filter_number: u8,
// ) -> ExtMsg {
//     let filter_index = filter_number as u32 * 8;
//     let extended_filter_element_address =
//         ram_base_address + extended_filter_list_start_address as u32 + filter_index;
//     ExtMsg::new(extended_filter_element_address as _)
// }

// impl GroupInterruptConfig {
//     pub fn is_enable_src(&self) -> bool {
//         self.priority > 0 || self.type_of_service == Tos::Dma
//     }
// }
