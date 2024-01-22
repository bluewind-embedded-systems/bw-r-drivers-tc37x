use core::marker::PhantomData;

use crate::can::baud_rate::{calculate_fast_bit_timing, BitTiming};
use crate::can::can_node::{
    FrameMode, Interrupt, InterruptGroup, InterruptLine, Priority, RxFifoMode, RxSel, Tos,
};
use crate::can::msg::{ReadFrom, RxBufferId, TxBufferId};
use crate::can::{DataFieldSize, Module, ModuleId, TxMode};
use tc37x_pac::can0::Can0;
use tc37x_pac::can1::Can1;
use tc37x_pac::RegisterValue;

use super::NodeId;

pub(super) struct NodeEffects<T> {
    pub(crate) reg: T,
    pub(crate) node_id: NodeId,
}

macro_rules! can_effects_node0 {
        ($Reg:ty) => {
        impl NodeEffects<$Reg>
        {
            pub(crate) fn new_node0<T>(module:$Reg) -> NodeEffects<$Reg> {
                NodeEffects::<$Reg>{reg:module, node_id:NodeId(0)}
            }

            pub(crate) fn set_rx_fifo0_data_field_size(&self, size: u8) {
                unsafe { self.reg.rxesc0().modify(|r| r.f0ds().set(size)) };
            }

           pub(crate) fn set_rx_fifo0_start_address(&self, address: u16) {
                unsafe { self.reg.rxf0c0().modify(|r| r.f0sa().set(address >> 2)) };
            }

           pub(crate) fn set_rx_fifo0_size(&self, size: u8) {
                unsafe { self.reg.rxf0c0().modify(|r| r.f0s().set(size)) };
            }

           pub(crate) fn set_rx_fifo0_watermark_level(&self, level: u8) {
                unsafe { self.reg.rxf0c0().modify(|r| r.f0wm().set(level)) };
            }

           pub(crate) fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
                unsafe {
                    self.reg
                        .rxf0c0()
                        .modify(|r| r.f0om().set(mode == RxFifoMode::Overwrite))
                };
            }

           pub(crate) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
                unsafe {
                    self.reg.txbtie0().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << tx_buffer_id.0;
                        r
                    })
                };
            }

            #[inline]
           pub(crate) fn set_dedicated_tx_buffers_number(&self, number: u8) {
                unsafe { self.reg.txbc0().modify(|r| r.ndtb().set(number)) };
            }

            #[inline]
           pub(crate) fn set_tx_event_fifo_start_address(&self, address: u16) {
                unsafe { self.reg.txefc0().modify(|r| r.efsa().set(address >> 2)) };
            }

            #[inline]
           pub(crate) fn set_tx_event_fifo_size(&self, size: u8) {
                unsafe { self.reg.txefc0().modify(|r| r.efs().set(size)) };
            }

           pub(crate) fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
                let val = (mode as u8) != 0;
                unsafe { self.reg.txbc0().modify(|r| r.tfqm().set(val)) };
            }

           pub(crate) fn set_transmit_fifo_queue_size(&self, number: u8) {
                unsafe { self.reg.txbc0().modify(|r| r.tfqs().set(number)) };
            }

           pub(crate) fn enable_configuration_change(&self) {
                let cccr = self.reg.cccr0();

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

           pub(crate) fn disable_configuration_change(&self) {
                let cccr = self.reg.cccr0();

                unsafe { cccr.modify(|r| r.cce().set(false)) };

                while unsafe { cccr.read() }.cce().get() {}

                unsafe { cccr.modify(|r| r.init().set(false)) };

                while unsafe { cccr.read() }.init().get() {}
            }

           pub(crate) fn set_nominal_bit_timing(&self, timing: &BitTiming) {
                unsafe {
                    self.reg.nbtp0().modify(|r| {
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

           pub(crate) fn set_data_bit_timing(&self, timing: &BitTiming) {
                // TODO Remove unwrap
                unsafe {
                    self.reg.dbtp0().modify(|r| {
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

           pub(crate) fn set_tx_buffer_data_field_size(&self, tdbs: u8) {
                unsafe { self.reg.txesc0().modify(|r| r.tbds().set(tdbs)) };
            }

           pub(crate) fn set_tx_buffer_start_address(&self, address: u16) {
                unsafe { self.reg.txbc0().modify(|r| r.tbsa().set(address >> 2)) };
            }

           pub(crate) fn set_frame_mode(&self, fdoe: bool, brse: bool) {
                unsafe {
                    self.reg
                        .cccr0()
                        .modify(|r| r.fdoe().set(fdoe).brse().set(brse))
                };
            }

           pub(crate) fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
                unsafe { self.reg.dbtp0().modify(|r| r.tdc().set(true)) };
                unsafe { self.reg.tdcr0().modify(|r| r.tdco().set(delay)) };
            }

           pub(crate) fn enable_interrupt(&self, interrupt: Interrupt) {
                unsafe {
                    self.reg.ie0().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << interrupt as u32;
                        r
                    })
                };
            }

           pub(crate) fn set_interrupt_routing_group_1(&self, line: u32, group: u32) {
                unsafe {
                    self.reg.grint10().modify(|mut r| {
                        *r.data_mut_ref() |= line << group;
                        r
                    })
                };
            }

           pub(crate) fn set_interrupt_routing_group_2(&self, line: u32, group: u32) {
                unsafe {
                    self.reg.grint20().modify(|mut r| {
                        *r.data_mut_ref() |= line << group;
                        r
                    })
                };
            }

           pub(crate) fn connect_pin_rx(&self, rx_sel: RxSel) {
                // TODO Check if "as u8" is necessary and safe
                unsafe { self.reg.npcr0().modify(|r| r.rxsel().set(rx_sel as u8)) };
            }

           pub(crate) fn get_rx_fifo0_fill_level(&self) -> u8 {
                unsafe { self.reg.rxf0s0().read()}.f0fl().get()
            }

           pub(crate) fn get_rx_fifo1_fill_level(&self) -> u8 {
                unsafe { self.reg.rxf1s0().read() }.f1fl().get()
            }

           pub(crate) fn set_rx_buffers_start_address(&self, address: u16) {
                unsafe { self.reg.rxbc0().modify(|r| r.rbsa().set(address >> 2)) };
            }

           pub(crate) fn set_rx_fifo1_size(&self, size: u8) {
                unsafe { self.reg.rxf1c0().modify(|r| r.f1s().set(size)) };
            }

           pub(crate) fn set_rx_fifo1_start_address(&self, address: u16) {
                unsafe { self.reg.rxf1c0().modify(|r| r.f1sa().set(address >> 2)) };
            }

           pub(crate) fn set_rx_fifo1_watermark_level(&self, level: u8) {
                unsafe { self.reg.rxf1c0().modify(|r| r.f1wm().set(level)) };
            }

           pub(crate) fn is_tx_event_fifo_element_lost(&self) -> bool {
                unsafe { self.reg.txefs0().read() }.tefl().get()
            }

           pub(crate) fn is_tx_event_fifo_full(&self) -> bool {
                unsafe { self.reg.txefs0().read() }.eff().get()
            }

           pub(crate) fn is_tx_fifo_queue_full(&self) -> bool {
                unsafe { self.reg.txfqs0().read() }.tfqf().get()
            }

           pub(crate) fn pause_trasmission(&self, enable: bool) {
                unsafe { self.reg.cccr0().modify(|r| r.txp().set(enable)) };
            }

           pub(crate) fn set_standard_filter_list_start_address(&self, address: u16) {
                unsafe { self.reg.sidfc0().modify(|r| r.flssa().set(address >> 2)) };
            }

           pub(crate) fn set_standard_filter_list_size(&self, size: u8) {
                unsafe { self.reg.sidfc0().modify(|r| r.lss().set(size)) };
            }

           pub(crate) fn reject_remote_frames_with_standard_id(&self) {
                unsafe { self.reg.gfc0().modify(|r| r.rrfs().set(true)) };
            }

           pub(crate) fn set_extended_filter_list_start_address(&self, address: u16) {
                unsafe { self.reg.xidfc0().modify(|r| r.flesa().set(address >> 2)) };
            }

           pub(crate) fn set_extended_filter_list_size(&self, size: u8) {
                unsafe { self.reg.xidfc0().modify(|r| r.lse().set(size)) };
            }

           pub(crate) fn reject_remote_frames_with_extended_id(&self) {
                unsafe { self.reg.gfc0().modify(|r| r.rrfe().set(true)) };
            }

           pub(crate) fn get_tx_fifo_queue_put_index(&self) -> u8 {
                unsafe { self.reg.txfqs0().read() }.tfqpi().get()
            }

           pub(crate) fn get_rx_fifo0_get_index(&self) -> u8 {
                unsafe { self.reg.rxf0s0().read() }.f0gi().get()
            }

           pub(crate) fn get_rx_fifo1_get_index(&self) -> u8 {
                unsafe { self.reg.rxf1s0().read() }.f1gi().get()
            }

           pub(crate) fn set_rx_buffer_data_field_size(&self, size: u8) {
                unsafe { self.reg.rxesc0().modify(|r| r.rbds().set(size)) };
                todo!()
            }

           pub(crate) fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: u8) -> bool {
                let (data, mask) = if rx_buffer_id < 32 {
                    // last number value in the reg name is the node id
                    let data = unsafe { self.reg.ndat10().read() }.data();
                    let mask = 1 << u8::from(rx_buffer_id);
                    (data, mask)
                } else {
                    // last number value in the reg name is the node id
                    let data = unsafe { self.reg.ndat20().read() }.data();
                    let mask = 1 << (u8::from(rx_buffer_id) - 32);
                    (data, mask)
                };
                (data & mask) != 0
            }

            #[inline]
           pub(crate) fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                unsafe {
                    self.reg
                        .rxf0a0()
                        .modify(|r| r.f0ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
           pub(crate) fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                unsafe {
                    self.reg
                        .rxf1a0()
                        .modify(|r| r.f1ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
           pub(crate) fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: u8) -> bool {
                let data = unsafe { self.reg.txbto0().read() }.data();
                let mask = 1u32 << u32::from(tx_buffer_id);
                (data & mask) != 0
            }

            #[inline]
           pub(crate) fn set_tx_buffer_add_request(&self/*,tx_buffer_id: TxBufferId*/) {
                unsafe {
                    self.reg
                        .txbar0()
                        // TODO argument is now a postfix?
                        .modify(|r| r.ar0(/*tx_buffer_id.into()*/).set(true))
                }
            }

            // TODO The original code does not work with current PAC
           pub(crate) fn get_data_field_size(&self, _from: ReadFrom) -> u8 {
                todo!();
                // let rx_esc = unsafe { self.reg.rxesc().read() };
                // let size_code:u32 = match from {
                //     ReadFrom::Buffer(_) => rx_esc.rbds().get().0,
                //     ReadFrom::RxFifo0 => rx_esc.f0ds().get().0,
                //     ReadFrom::RxFifo1 => rx_esc.f1ds().get().0,
                // };

                // if size_code < DataFieldSize::_32.into() {
                //     (size_code + 2) * 4
                // } else {
                //     (size_code - 3) * 16
                // }
            }

           pub(crate) fn get_tx_buffer_data_field_size(&self) -> u8 {
                let size_code: u8 = (unsafe { self.reg.txesc0().read() }.get_raw() & 0x2) as u8;
                if size_code < (DataFieldSize::_32 as u8) {
                    (size_code + 2) * 4
                } else {
                    (size_code - 3) * 16
                }
            }

            pub(crate) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
                match(tx_buffer_id.0) {
                    0 =>  unsafe { self.reg.txbrp0().read() }.trp0().get(),
                    1 =>  unsafe { self.reg.txbrp0().read() }.trp1().get(),
                    2 =>  unsafe { self.reg.txbrp0().read() }.trp2().get(),
                    3 =>  unsafe { self.reg.txbrp0().read() }.trp3().get(),
                    4 =>  unsafe { self.reg.txbrp0().read() }.trp4().get(),
                    5 =>  unsafe { self.reg.txbrp0().read() }.trp5().get(),
                    6 =>  unsafe { self.reg.txbrp0().read() }.trp6().get(),
                    7 =>  unsafe { self.reg.txbrp0().read() }.trp7().get(),
                    8 =>  unsafe { self.reg.txbrp0().read() }.trp8().get(),
                    // TODO implement trp0..trp31
                    _ => todo!()
                }
    
            }
               
        }
}
}

macro_rules! can_effects_node1 {
    ($Reg:ty) => {
    impl NodeEffects<$Reg>
    {
        pub(crate) fn new_node0<T>(module:$Reg) -> NodeEffects<$Reg> {
            NodeEffects::<$Reg>{reg:module, node_id:NodeId(0)}
        }

        pub(crate) fn set_rx_fifo0_data_field_size(&self, size: u8) {
            unsafe { self.reg.rxesc1().modify(|r| r.f0ds().set(size)) };
        }

       pub(crate) fn set_rx_fifo0_start_address(&self, address: u16) {
            unsafe { self.reg.rxf0c1().modify(|r| r.f0sa().set(address >> 2)) };
        }

       pub(crate) fn set_rx_fifo0_size(&self, size: u8) {
            unsafe { self.reg.rxf0c1().modify(|r| r.f0s().set(size)) };
        }

       pub(crate) fn set_rx_fifo0_watermark_level(&self, level: u8) {
            unsafe { self.reg.rxf0c1().modify(|r| r.f0wm().set(level)) };
        }

       pub(crate) fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
            unsafe {
                self.reg
                    .rxf0c1()
                    .modify(|r| r.f0om().set(mode == RxFifoMode::Overwrite))
            };
        }

       pub(crate) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
            unsafe {
                self.reg.txbtie1().modify(|mut r| {
                    *r.data_mut_ref() |= 1 << tx_buffer_id.0;
                    r
                })
            };
        }

        #[inline]
       pub(crate) fn set_dedicated_tx_buffers_number(&self, number: u8) {
            unsafe { self.reg.txbc1().modify(|r| r.ndtb().set(number)) };
        }

        #[inline]
       pub(crate) fn set_tx_event_fifo_start_address(&self, address: u16) {
            unsafe { self.reg.txefc1().modify(|r| r.efsa().set(address >> 2)) };
        }

        #[inline]
       pub(crate) fn set_tx_event_fifo_size(&self, size: u8) {
            unsafe { self.reg.txefc1().modify(|r| r.efs().set(size)) };
        }

       pub(crate) fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
            let val = (mode as u8) != 0;
            unsafe { self.reg.txbc1().modify(|r| r.tfqm().set(val)) };
        }

       pub(crate) fn set_transmit_fifo_queue_size(&self, number: u8) {
            unsafe { self.reg.txbc1().modify(|r| r.tfqs().set(number)) };
        }

       pub(crate) fn enable_configuration_change(&self) {
            let cccr = self.reg.cccr1();

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

       pub(crate) fn disable_configuration_change(&self) {
            let cccr = self.reg.cccr1();

            unsafe { cccr.modify(|r| r.cce().set(false)) };

            while unsafe { cccr.read() }.cce().get() {}

            unsafe { cccr.modify(|r| r.init().set(false)) };

            while unsafe { cccr.read() }.init().get() {}
        }

       pub(crate) fn set_nominal_bit_timing(&self, timing: &BitTiming) {
            unsafe {
                self.reg.nbtp1().modify(|r| {
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

       pub(crate) fn set_data_bit_timing(&self, timing: &BitTiming) {
            // TODO Remove unwrap
            unsafe {
                self.reg.dbtp1().modify(|r| {
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

       pub(crate) fn set_tx_buffer_data_field_size(&self, tdbs: u8) {
            unsafe { self.reg.txesc1().modify(|r| r.tbds().set(tdbs)) };
        }

       pub(crate) fn set_tx_buffer_start_address(&self, address: u16) {
            unsafe { self.reg.txbc1().modify(|r| r.tbsa().set(address >> 2)) };
        }

       pub(crate) fn set_frame_mode(&self, fdoe: bool, brse: bool) {
            unsafe {
                self.reg
                    .cccr1()
                    .modify(|r| r.fdoe().set(fdoe).brse().set(brse))
            };
        }

       pub(crate) fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
            unsafe { self.reg.dbtp1().modify(|r| r.tdc().set(true)) };
            unsafe { self.reg.tdcr1().modify(|r| r.tdco().set(delay)) };
        }

       pub(crate) fn enable_interrupt(&self, interrupt: Interrupt) {
            unsafe {
                self.reg.ie1().modify(|mut r| {
                    *r.data_mut_ref() |= 1 << interrupt as u32;
                    r
                })
            };
        }

       pub(crate) fn set_interrupt_routing_group_1(&self, line: u32, group: u32) {
            unsafe {
                self.reg.grint11().modify(|mut r| {
                    *r.data_mut_ref() |= line << group;
                    r
                })
            };
        }

       pub(crate) fn set_interrupt_routing_group_2(&self, line: u32, group: u32) {
            unsafe {
                self.reg.grint21().modify(|mut r| {
                    *r.data_mut_ref() |= line << group;
                    r
                })
            };
        }

       pub(crate) fn connect_pin_rx(&self, rx_sel: RxSel) {
            // TODO Check if "as u8" is necessary and safe
            unsafe { self.reg.npcr1().modify(|r| r.rxsel().set(rx_sel as u8)) };
        }

       pub(crate) fn get_rx_fifo0_fill_level(&self) -> u8 {
            unsafe { self.reg.rxf0s1().read()}.f0fl().get()
        }

       pub(crate) fn get_rx_fifo1_fill_level(&self) -> u8 {
            unsafe { self.reg.rxf1s1().read() }.f1fl().get()
        }

       pub(crate) fn set_rx_buffers_start_address(&self, address: u16) {
            unsafe { self.reg.rxbc1().modify(|r| r.rbsa().set(address >> 2)) };
        }

       pub(crate) fn set_rx_fifo1_size(&self, size: u8) {
            unsafe { self.reg.rxf1c1().modify(|r| r.f1s().set(size)) };
        }

       pub(crate) fn set_rx_fifo1_start_address(&self, address: u16) {
            unsafe { self.reg.rxf1c1().modify(|r| r.f1sa().set(address >> 2)) };
        }

       pub(crate) fn set_rx_fifo1_watermark_level(&self, level: u8) {
            unsafe { self.reg.rxf1c1().modify(|r| r.f1wm().set(level)) };
        }

       pub(crate) fn is_tx_event_fifo_element_lost(&self) -> bool {
            unsafe { self.reg.txefs1().read() }.tefl().get()
        }

       pub(crate) fn is_tx_event_fifo_full(&self) -> bool {
            unsafe { self.reg.txefs1().read() }.eff().get()
        }

       pub(crate) fn is_tx_fifo_queue_full(&self) -> bool {
            unsafe { self.reg.txfqs1().read() }.tfqf().get()
        }

       pub(crate) fn pause_trasmission(&self, enable: bool) {
            unsafe { self.reg.cccr1().modify(|r| r.txp().set(enable)) };
        }

       pub(crate) fn set_standard_filter_list_start_address(&self, address: u16) {
            unsafe { self.reg.sidfc1().modify(|r| r.flssa().set(address >> 2)) };
        }

       pub(crate) fn set_standard_filter_list_size(&self, size: u8) {
            unsafe { self.reg.sidfc1().modify(|r| r.lss().set(size)) };
        }

       pub(crate) fn reject_remote_frames_with_standard_id(&self) {
            unsafe { self.reg.gfc1().modify(|r| r.rrfs().set(true)) };
        }

       pub(crate) fn set_extended_filter_list_start_address(&self, address: u16) {
            unsafe { self.reg.xidfc1().modify(|r| r.flesa().set(address >> 2)) };
        }

       pub(crate) fn set_extended_filter_list_size(&self, size: u8) {
            unsafe { self.reg.xidfc1().modify(|r| r.lse().set(size)) };
        }

       pub(crate) fn reject_remote_frames_with_extended_id(&self) {
            unsafe { self.reg.gfc1().modify(|r| r.rrfe().set(true)) };
        }

       pub(crate) fn get_tx_fifo_queue_put_index(&self) -> u8 {
            unsafe { self.reg.txfqs1().read() }.tfqpi().get()
        }

       pub(crate) fn get_rx_fifo0_get_index(&self) -> u8 {
            unsafe { self.reg.rxf0s1().read() }.f0gi().get()
        }

       pub(crate) fn get_rx_fifo1_get_index(&self) -> u8 {
            unsafe { self.reg.rxf1s1().read() }.f1gi().get()
        }

       pub(crate) fn set_rx_buffer_data_field_size(&self, size: u8) {
            unsafe { self.reg.rxesc1().modify(|r| r.rbds().set(size)) };
            todo!()
        }

       pub(crate) fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: u8) -> bool {
            let (data, mask) = if rx_buffer_id < 32 {
                // last number value in the reg name is the node id
                let data = unsafe { self.reg.ndat11().read() }.data();
                let mask = 1 << u8::from(rx_buffer_id);
                (data, mask)
            } else {
                // last number value in the reg name is the node id
                let data = unsafe { self.reg.ndat21().read() }.data();
                let mask = 1 << (u8::from(rx_buffer_id) - 32);
                (data, mask)
            };
            (data & mask) != 0
        }

        #[inline]
       pub(crate) fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
            unsafe {
                self.reg
                    .rxf0a1()
                    .modify(|r| r.f0ai().set(rx_buffer_id.into()))
            };
        }

        #[inline]
       pub(crate) fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
            unsafe {
                self.reg
                    .rxf1a1()
                    .modify(|r| r.f1ai().set(rx_buffer_id.into()))
            };
        }

        #[inline]
       pub(crate) fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: u8) -> bool {
            let data = unsafe { self.reg.txbto1().read() }.data();
            let mask = 1u32 << u32::from(tx_buffer_id);
            (data & mask) != 0
        }

        #[inline]
       pub(crate) fn set_tx_buffer_add_request(&self/*,tx_buffer_id: TxBufferId*/) {
            unsafe {
                self.reg
                    .txbar1()
                    // TODO argument is now a postfix?
                    .modify(|r| r.ar1(/*tx_buffer_id.into()*/).set(true))
            }
        }

        // TODO The original code does not work with current PAC
       pub(crate) fn get_data_field_size(&self, _from: ReadFrom) -> u8 {
            todo!();
            // let rx_esc = unsafe { self.reg.rxesc().read() };
            // let size_code:u32 = match from {
            //     ReadFrom::Buffer(_) => rx_esc.rbds().get().0,
            //     ReadFrom::RxFifo0 => rx_esc.f0ds().get().0,
            //     ReadFrom::RxFifo1 => rx_esc.f1ds().get().0,
            // };

            // if size_code < DataFieldSize::_32.into() {
            //     (size_code + 2) * 4
            // } else {
            //     (size_code - 3) * 16
            // }
        }

       pub(crate) fn get_tx_buffer_data_field_size(&self) -> u8 {
            let size_code: u8 = (unsafe { self.reg.txesc1().read() }.get_raw() & 0x2) as u8;
            if size_code < (DataFieldSize::_32 as u8) {
                (size_code + 2) * 4
            } else {
                (size_code - 3) * 16
            }
        }

       pub(crate) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
            match(tx_buffer_id.0) {
                0 =>  unsafe { self.reg.txbrp1().read() }.trp0().get(),
                1 =>  unsafe { self.reg.txbrp1().read() }.trp1().get(),
                2 =>  unsafe { self.reg.txbrp1().read() }.trp2().get(),
                3 =>  unsafe { self.reg.txbrp1().read() }.trp3().get(),
                4 =>  unsafe { self.reg.txbrp1().read() }.trp4().get(),
                5 =>  unsafe { self.reg.txbrp1().read() }.trp5().get(),
                6 =>  unsafe { self.reg.txbrp1().read() }.trp6().get(),
                7 =>  unsafe { self.reg.txbrp1().read() }.trp7().get(),
                8 =>  unsafe { self.reg.txbrp1().read() }.trp8().get(),
                // TODO implement trp0..trp31
                _ => todo!()
            }

        }
    }
}
}

can_effects_node0!(Can0);
can_effects_node1!(Can1);
