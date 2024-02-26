// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::baud_rate::{DataBitTiming, NominalBitTiming};
use crate::can::can_node::{Interrupt, RxFifoMode, RxSel};
use crate::can::msg::{ReadFrom, RxBufferId, TxBufferId};
use crate::can::{DataFieldSize, TxMode};
use crate::pac;

use tc37x_pac::hidden::RegValue;
use tc37x_pac::RegisterValue;

pub(super) struct NodeEffects<T> {
    reg: T,
}

macro_rules! impl_can_node_effect {
    ($NodeReg:path) => {
        impl NodeEffects<$NodeReg> {
            pub(super) fn new(reg: $NodeReg) -> NodeEffects<$NodeReg> {
                NodeEffects { reg }
            }

            pub(crate) fn set_rx_buffer_data_field_size(&self, size: DataFieldSize) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 3, 7, 31:11 are written with 0, size is in range [0, 7]
                unsafe {
                    self.reg
                        .rx()
                        .rxesci()
                        .modify(|r| r.rbds().set(size.to_esci_register_value().into()))
                };
            }

            pub(crate) fn set_rx_fifo0_data_field_size(&self, size: DataFieldSize) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 3, 7, 31:11 are written with 0, size is in range [0, 7]
                unsafe {
                    self.reg
                        .rx()
                        .rxesci()
                        .modify(|r| r.f0ds().set(size.to_esci_register_value().into()))
                };
            }

            pub(crate) fn set_rx_fifo1_data_field_size(&self, size: DataFieldSize) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 3, 7, 31:11 are written with 0, size is in range [0, 7]
                unsafe {
                    self.reg
                        .rx()
                        .rxesci()
                        .modify(|r| r.f1ds().set(size.to_esci_register_value().into()))
                };
            }

            pub(crate) fn set_rx_fifo0_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO address should be in range [0, 2^14)
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ci()
                        .modify(|r| r.f0sa().set(address >> 2))
                };
            }

            pub(crate) fn set_rx_fifo0_size(&self, size: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO size should be in range [0, 2^7)
                unsafe { self.reg.rx().rxf0ci().modify(|r| r.f0s().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo0_watermark_level(&self, level: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO level should be in range [0, 2^7) 
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ci()
                        .modify(|r| r.f0wm().set(level.into()))
                };
            }

            // TODO Move logic to the caller
            pub(crate) fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
                let overwrite = mode == RxFifoMode::Overwrite;
                let overwrite = u8::from(overwrite);
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, overwrite is in range [0, 1]
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ci()
                        .modify(|r| r.f0om().set(overwrite.into()))
                };
            }

            // TODO Move logic to the caller
            pub(crate) fn set_rx_fifo1_operating_mode(&self, mode: RxFifoMode) {
                let overwrite = mode == RxFifoMode::Overwrite;
                let overwrite = u8::from(overwrite);
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, overwrite is in range [0, 1]
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1om().set(overwrite.into()))
                };
            }

            pub(crate) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
                let id: u8 = tx_buffer_id.into();
                // SAFETY: each bit is RW, TODO tx_buffer_id should be in range [0, 31], use try_from?
                unsafe {
                    self.reg.tx().txbtiei().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << id;
                        r
                    })
                };
            }

            #[inline]
            pub(crate) fn set_dedicated_tx_buffers_number(&self, number: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31 are written with 0, TODO number should be in range [0, 63]
                unsafe {
                    self.reg
                        .tx()
                        .txbci()
                        .modify(|r| r.ndtb().set(number.into()))
                };
            }

            #[inline]
            pub(crate) fn set_tx_event_fifo_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31:30 are written with 0, TODO address should be in range [0, 2^14)
                unsafe {
                    self.reg
                        .tx()
                        .txefci()
                        .modify(|r| r.efsa().set(address >> 2))
                };
            }

            #[inline]
            pub(crate) fn set_tx_event_fifo_size(&self, size: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31:30 are written with 0, TODO size should be in range [0, 2^7)
                unsafe { self.reg.tx().txefci().modify(|r| r.efs().set(size.into())) };
            }

            // TODO Move logic to the caller
            pub(crate) fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
                let val = mode != TxMode::DedicatedBuffers;
                let val = u8::from(val);
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31:30 are written with 0, val is in range [0, 1]
                unsafe { self.reg.tx().txbci().modify(|r| r.tfqm().set(val.into())) };
            }

            pub(crate) fn set_transmit_fifo_queue_size(&self, number: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31:30 are written with 0, TODO number should be in range [0, 2^7)
                unsafe {
                    self.reg
                        .tx()
                        .txbci()
                        .modify(|r| r.tfqs().set(number.into()))
                };
            }

            pub(crate) fn get_rx_element_address(
                &self,
                ram_base_address: u32,
                tx_buffers_start_address: u16,
                buf_from: ReadFrom,
                buffer_number: RxBufferId,
            ) -> crate::can::internals::Rx {
                let num_of_config_bytes = 8u32;
                let num_of_data_bytes = self.get_data_field_size(buf_from) as u32;
                let tx_buffer_size = num_of_config_bytes + num_of_data_bytes;
                let tx_buffer_index = tx_buffer_size * u32::from(buffer_number);

                let tx_buffer_element_address =
                    ram_base_address + tx_buffers_start_address as u32 + tx_buffer_index;

                crate::can::internals::Rx::new(tx_buffer_element_address as *mut u8)
            }

            pub(crate) fn clear_rx_buffer_new_data_flag(&self, rx_buffer_id: RxBufferId) {
                if u8::from(rx_buffer_id) < 32u8 {
                    // SAFETY: rx_buffer_id is between 0 and 31
                    unsafe {
                        self.reg
                            .ndat1i()
                            .init(|r| r.set_raw(1u32 << (u8::from(rx_buffer_id))));

                        // TODO A (safer?) alternative is being more explicit. Discuss about it.
                        // self.reg.ndat1i().init(|r| match rx_buffer_id.0 {
                        //     0 => r.nd0().set(1u8.into()),
                        //     1 => r.nd1().set(1u8.into()),
                        //     2 => r.nd2().set(1u8.into()),
                        //     3 => r.nd3().set(1u8.into()),
                        //     4 => r.nd4().set(1u8.into()),
                        //     5 => r.nd5().set(1u8.into()),
                        //     6 => r.nd6().set(1u8.into()),
                        //     7 => r.nd7().set(1u8.into()),
                        //     8 => r.nd8().set(1u8.into()),
                        //     9 => r.nd9().set(1u8.into()),
                        //     10 => r.nd10().set(1u8.into()),
                        //     11 => r.nd11().set(1u8.into()),
                        //     12 => r.nd12().set(1u8.into()),
                        //     13 => r.nd13().set(1u8.into()),
                        //     14 => r.nd14().set(1u8.into()),
                        //     15 => r.nd15().set(1u8.into()),
                        //     16 => r.nd16().set(1u8.into()),
                        //     17 => r.nd17().set(1u8.into()),
                        //     18 => r.nd18().set(1u8.into()),
                        //     19 => r.nd19().set(1u8.into()),
                        //     20 => r.nd20().set(1u8.into()),
                        //     21 => r.nd21().set(1u8.into()),
                        //     22 => r.nd22().set(1u8.into()),
                        //     23 => r.nd23().set(1u8.into()),
                        //     24 => r.nd24().set(1u8.into()),
                        //     25 => r.nd25().set(1u8.into()),
                        //     26 => r.nd26().set(1u8.into()),
                        //     27 => r.nd27().set(1u8.into()),
                        //     28 => r.nd28().set(1u8.into()),
                        //     29 => r.nd29().set(1u8.into()),
                        //     30 => r.nd30().set(1u8.into()),
                        //     31 => r.nd31().set(1u8.into()),
                        //     _ => unreachable!(),
                        // });
                    };
                } else {
                    // SAFETY: rx_buffer_id is between 32 and 63
                    unsafe {
                        self.reg
                            .ndat2i()
                            .init(|r| r.set_raw(1u32 << (u8::from(rx_buffer_id) - 32)));
                    };
                }
            }

            // TODO Return a different type which implements methods needing configuration change enabled
            pub(crate) fn enable_configuration_change(&self) {
                let cccr = self.reg.cccri();

                // SAFETY: INIT bit is RWH
                if unsafe { cccr.read() }.init().get().0 == 1u8 {
                    // SAFETY: CCE bit is RW
                    unsafe { cccr.modify(|r| r.cce().set(0u8.into())) };
                    while {
                        // SAFETY: CCE bit is RW
                        unsafe { cccr.read() }.cce().get().0 != 0u8
                    } {}
                    // SAFETY: INIT bit is RWH
                    unsafe { cccr.modify(|r| r.init().set(0u8.into())) };
                    while {
                        // SAFETY: INIT bit is RWH
                        unsafe { cccr.read() }.init().get().0 != 0u8
                    } {}
                }

                // SAFETY: INIT bit is RWH
                unsafe { cccr.modify(|r| r.init().set(1u8.into())) };
                while {
                    // SAFETY: INIT bit is RWH
                    unsafe { cccr.read() }.init().get().0 != 1u8
                } {}

                // SAFETY: INIT bit is RWH, CCE bit is RW
                unsafe { cccr.modify(|r| r.cce().set(1u8.into()).init().set(1u8.into())) };
            }

            // TODO Return a different type which does not implement methods needing configuration change enabled
            pub(crate) fn disable_configuration_change(&self) {
                let cccr = self.reg.cccri();

                // SAFETY: CCE bit is RW
                unsafe { cccr.modify(|r| r.cce().set(0u8.into())) };
                while {
                    // SAFETY: CCE bit is RW
                    unsafe { cccr.read() }.cce().get().0 != 0u8
                } {}

                // SAFETY: INIT bit is RWH
                unsafe { cccr.modify(|r| r.init().set(0u8.into())) };
                while {
                    // SAFETY: INIT bit is RWH
                    unsafe { cccr.read() }.init().get().0 != 0u8
                } {}
            }

            pub(crate) fn set_nominal_bit_timing(&self, timing: &NominalBitTiming) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.configure_baud_rate after node.effects.enable_configuration_change has been called in Node::new.
                // bit 7 is written with 0, TODO timing.brp should be in range [0, 2^9)
                // timing.sjw should be in range [0, 2^7)
                // timing.tseg1 should be in range [0, 2^8)
                // timing.tseg2 should be in range [0, 2^7)
                unsafe {
                    self.reg.nbtpi().modify(|r| {
                        r.nbrp()
                            .set(timing.brp as u16) // expected u16
                            .nsjw()
                            .set(timing.sjw)
                            .ntseg1()
                            .set(timing.tseg1)
                            .ntseg2()
                            .set(timing.tseg2)
                    })
                }
            }

            pub(crate) fn set_data_bit_timing(&self, timing: &DataBitTiming) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.configure_fast_baud_rate after node.effects.enable_configuration_change has been called in Node::new.
                // bits 15:13, 22:21 and 31:24 are written with 0, TODO timing.brp should be in range [0, 2^5)
                // timing.sjw should be in range [0, 2^4)
                // timing.tseg1 should be in range [0, 2^5)
                // timing.tseg2 should be in range [0, 2^4)
                unsafe {
                    self.reg.dbtpi().modify(|r| {
                        r.dbrp()
                            .set(timing.brp)
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
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 31:3 are written with 0, TODO tdbs should be in range [0, 2^3)
                unsafe { self.reg.tx().txesci().modify(|r| r.tbds().set(tdbs.into())) };
            }

            pub(crate) fn set_tx_buffer_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_tx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23:22 and 31 are written with 0, TODO address should be in range [0, 2^14)
                unsafe { self.reg.tx().txbci().modify(|r| r.tbsa().set(address >> 2)) };
            }

            pub(crate) fn set_rx_buffer_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0 and 31:16 are written with 0, TODO address should be in range [0, 2^14)
                unsafe { self.reg.rx().rxbci().modify(|r| r.rbsa().set(address >> 2)) };
            }

            pub(crate) fn set_frame_mode(&self, fdoe: bool, brse: bool) {
                let fdoe = u8::from(fdoe);
                let brse = u8::from(brse);
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>. after node.effects.enable_configuration_change has been called in Node::new.
                // bits 11:10 and 31:16 are written with 0, fdoe and brse are in range [0, 1]
                unsafe {
                    self.reg
                        .cccri()
                        .modify(|r| r.fdoe().set(fdoe.into()).brse().set(brse.into()))
                };
            }

            pub(crate) fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
                // SAFETY: write is CCE and INIT protected: called after node.effects.enable_configuration_change has been called in Node::new.
                // bits 15:13, 22:21 and 31:24 are written with 0, TDC bit is RW
                unsafe { self.reg.dbtpi().modify(|r| r.tdc().set(1u8.into())) };
                // SAFETY: write is CCE and INIT protected: called after node.effects.enable_configuration_change has been called in Node::new.
                // bits 7 and 31:15 are written with 0, TODO delay should be in range [0, 2^7)
                unsafe { self.reg.tdcri().modify(|r| r.tdco().set(delay)) };
            }

            pub(crate) fn enable_interrupt(&self, interrupt: Interrupt) {
                // SAFETY: bits 20, 21, 29 and 31:30 are written with 0, interrupt is guaranteed to take only allowed values
                unsafe {
                    self.reg.iei().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << interrupt as u32;
                        r
                    })
                };
            }

            #[inline]
            pub(crate) fn clear_interrupt_flag(&self, interrupt: Interrupt) {
                // SAFETY: bits 20, 21, 29 and 31:30 are written with 0, interrupt is guaranteed to take only allowed values
                unsafe {
                    self.reg.iri().init(|mut r| {
                        *r.data_mut_ref() = 1 << interrupt as u32;
                        r
                    })
                };
            }

            pub(crate) fn set_interrupt_routing_group_1(&self, line: u32, group: u32) {
                unsafe {
                    self.reg.grint1i().modify(|mut r| {
                        *r.data_mut_ref() |= line << group;
                        r
                    })
                };
            }

            pub(crate) fn set_interrupt_routing_group_2(&self, line: u32, group: u32) {
                unsafe {
                    self.reg.grint2i().modify(|mut r| {
                        *r.data_mut_ref() |= line << group;
                        r
                    })
                };
            }

            pub(crate) fn connect_pin_rx(&self, rx_sel: RxSel) {
                // SAFETY: bits 7:3 and 31:11 are written with 0, rx_sel is guaranteed to take only allowed values
                unsafe { self.reg.npcri().modify(|r| r.rxsel().set(rx_sel.into())) };
            }

            pub(crate) fn get_rx_fifo0_fill_level(&self) -> u8 {
                // SAFETY: F0FL is RH
                unsafe { self.reg.rx().rxf0si().read() }.f0fl().get()
            }

            pub(crate) fn get_rx_fifo1_fill_level(&self) -> u8 {
                // SAFETY: F1FL is RH
                unsafe { self.reg.rx().rxf1si().read() }.f1fl().get()
            }

            pub(crate) fn set_rx_fifo1_size(&self, size: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO size should be in range [0, 2^7)
                unsafe { self.reg.rx().rxf1ci().modify(|r| r.f1s().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo1_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO address should be in range [0, 2^14)
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1sa().set(address >> 2))
                };
            }

            pub(crate) fn set_rx_fifo1_watermark_level(&self, level: u8) {
                // SAFETY: write is CCE and INIT protected: called in Node<Configurable>.setup_rx after node.effects.enable_configuration_change has been called in Node::new.
                // bits 1:0, 23 are written with 0, TODO level should be in range [0, 2^7)
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1wm().set(level.into()))
                };
            }

            pub(crate) fn is_tx_event_fifo_element_lost(&self) -> bool {
                // SAFETY: TEFL is RH
                unsafe { self.reg.tx().txefsi().read() }.tefl().get().0 == 1
            }

            pub(crate) fn is_tx_event_fifo_full(&self) -> bool {
                // SAFETY: EFF is RH
                unsafe { self.reg.tx().txefsi().read() }.eff().get().0 == 1
            }

            pub(crate) fn is_tx_fifo_queue_full(&self) -> bool {
                // SAFETY: TFQF is RH
                unsafe { self.reg.tx().txfqsi().read() }.tfqf().get().0 == 1
            }

            pub(crate) fn pause_trasmission(&self, enable: bool) {
                // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 11:10 and 31:16 are written with 0, enable is in range [0, 1]
                unsafe {
                    self.reg
                        .cccri()
                        .modify(|r| r.txp().set(u8::from(enable).into()))
                };
            }

            pub(crate) fn set_standard_filter_list_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 1:0 and 31:24 are written with 0, TODO: address should be in range [0, 2^14)
                unsafe { self.reg.sidfci().modify(|r| r.flssa().set(address >> 2)) };
            }

            pub(crate) fn set_standard_filter_list_size(&self, size: u8) {
                // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 1:0 and 31:24 are written with 0, size is in range [0, 2^8)
                unsafe { self.reg.sidfci().modify(|r| r.lss().set(size.into())) };
            }

            pub(crate) fn reject_remote_frames_with_standard_id(&self) {
                 // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 31:6 are written with 0, RRFS is a RW bit
                unsafe {
                    self.reg
                        .gfci()
                        .modify(|r| r.rrfs().set(u8::from(true).into()))
                };
            }

            pub(crate) fn set_extended_filter_list_start_address(&self, address: u16) {
                // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 1:0 and 31:24 are written with 0, TODO: address should be in range [0, 2^14)
                unsafe { self.reg.xidfci().modify(|r| r.flesa().set(address >> 2)) };
            }

            pub(crate) fn set_extended_filter_list_size(&self, size: u8) {
                // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 1:0 and 31:24 are written with 0, size is in range [0, 2^8)
                unsafe {
                    self.reg
                        .xidfci()
                        .modify(|r| r.lse().set(u8::from(size).into()))
                };
            }

            pub(crate) fn reject_remote_frames_with_extended_id(&self) {
                 // SAFETY: write is CCE and INIT protected: TODO: never used
                // bits 31:6 are written with 0, RRFE is a RW bit
                unsafe { self.reg.gfci().modify(|r| r.rrfe().set(1u8.into())) };
            }

            pub(crate) fn get_tx_fifo_queue_put_index(&self) -> u8 {
                // SAFETY: TFQPI is RH
                unsafe { self.reg.tx().txfqsi().read() }.tfqpi().get()
            }

            pub(crate) fn get_rx_fifo0_get_index(&self) -> RxBufferId {
                // SAFETY: F0GI is RH
                let idx: u8 = unsafe { self.reg.rx().rxf0si().read() }.f0gi().get();
                // SAFETY: idx is always between 0 and 63
                unsafe { RxBufferId::new_unchecked(idx) }
            }

            pub(crate) fn get_rx_fifo1_get_index(&self) -> RxBufferId {
                // SAFETY: F1GI is RH
                let idx: u8 = unsafe { self.reg.rx().rxf1si().read() }.f1gi().get();
                // SAFETY: idx is always between 0 and 63
                unsafe { RxBufferId::new_unchecked(idx) }
            }

            pub(crate) fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: u8) -> bool {
                let (data, mask) = if rx_buffer_id < 32 {
                    // last number value in the reg name is the node id
                    // SAFETY: each bit of NDAT1i is RWH
                    let data = unsafe { self.reg.ndat1i().read() }.data();
                    let mask = 1 << u8::from(rx_buffer_id);
                    (data, mask)
                } else {
                    // last number value in the reg name is the node id
                    // SAFETY: each bit of NDAT2i is RWH
                    let data = unsafe { self.reg.ndat2i().read() }.data();
                    let mask = 1 << (u8::from(rx_buffer_id) - 32);
                    (data, mask)
                };
                (data & mask) != 0
            }

            #[inline]
            pub(crate) fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                // SAFETY: bits 31:6 are written with 0, TODO: rx_buffer_id should be in range [0, 2^6)
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ai()
                        .modify(|r| r.f0ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
            pub(crate) fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                // SAFETY: bits 31:6 are written with 0, TODO: rx_buffer_id should be in range [0, 2^6)
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ai()
                        .modify(|r| r.f1ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
            pub(crate) fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: u8) -> bool {
                // SAFETY: each bit of TXBTOI is RH
                let data = unsafe { self.reg.tx().txbtoi().read() }.data();
                let mask = 1u32 << u32::from(tx_buffer_id);
                (data & mask) != 0
            }

            #[inline]
            pub(crate) fn set_tx_buffer_add_request(&self, id: u8) {
                let txbari = self.reg.tx().txbari();
                match id {
                    // SAFETY: AR0 is a RWH bit
                    0 => unsafe { txbari.modify(|r| r.ar0().set(1u8.into())) },
                    // SAFETY: AR1 is a RWH bit
                    1 => unsafe { txbari.modify(|r| r.ar1().set(1u8.into())) },
                    // SAFETY: AR2 is a RWH bit
                    2 => unsafe { txbari.modify(|r| r.ar2().set(1u8.into())) },
                    // SAFETY: AR3 is a RWH bit
                    3 => unsafe { txbari.modify(|r| r.ar3().set(1u8.into())) },
                    // SAFETY: AR4 is a RWH bit
                    4 => unsafe { txbari.modify(|r| r.ar4().set(1u8.into())) },
                    // SAFETY: AR5 is a RWH bit
                    5 => unsafe { txbari.modify(|r| r.ar5().set(1u8.into())) },
                    // SAFETY: AR6 is a RWH bit
                    6 => unsafe { txbari.modify(|r| r.ar6().set(1u8.into())) },
                    // SAFETY: AR7 is a RWH bit
                    7 => unsafe { txbari.modify(|r| r.ar7().set(1u8.into())) },
                    // SAFETY: AR8 is a RWH bit
                    8 => unsafe { txbari.modify(|r| r.ar8().set(1u8.into())) },
                    // SAFETY: AR9 is a RWH bit
                    9 => unsafe { txbari.modify(|r| r.ar9().set(1u8.into())) },
                    // SAFETY: AR10 is a RWH bit
                    10 => unsafe { txbari.modify(|r| r.ar10().set(1u8.into())) },
                    // SAFETY: AR11 is a RWH bit
                    11 => unsafe { txbari.modify(|r| r.ar11().set(1u8.into())) },
                    // SAFETY: AR12 is a RWH bit
                    12 => unsafe { txbari.modify(|r| r.ar12().set(1u8.into())) },
                    // SAFETY: AR13 is a RWH bit
                    13 => unsafe { txbari.modify(|r| r.ar13().set(1u8.into())) },
                    // SAFETY: AR14 is a RWH bit
                    14 => unsafe { txbari.modify(|r| r.ar14().set(1u8.into())) },
                    // SAFETY: AR15 is a RWH bit
                    15 => unsafe { txbari.modify(|r| r.ar15().set(1u8.into())) },
                    // SAFETY: AR16 is a RWH bit
                    16 => unsafe { txbari.modify(|r| r.ar16().set(1u8.into())) },
                    // SAFETY: AR17 is a RWH bit
                    17 => unsafe { txbari.modify(|r| r.ar17().set(1u8.into())) },
                    // SAFETY: AR18 is a RWH bit
                    18 => unsafe { txbari.modify(|r| r.ar18().set(1u8.into())) },
                    // SAFETY: AR19 is a RWH bit
                    19 => unsafe { txbari.modify(|r| r.ar19().set(1u8.into())) },
                    // SAFETY: AR20 is a RWH bit
                    20 => unsafe { txbari.modify(|r| r.ar20().set(1u8.into())) },
                    // SAFETY: AR21 is a RWH bit
                    21 => unsafe { txbari.modify(|r| r.ar21().set(1u8.into())) },
                    // SAFETY: AR22 is a RWH bit
                    22 => unsafe { txbari.modify(|r| r.ar22().set(1u8.into())) },
                    // SAFETY: AR23 is a RWH bit
                    23 => unsafe { txbari.modify(|r| r.ar23().set(1u8.into())) },
                    // SAFETY: AR24 is a RWH bit
                    24 => unsafe { txbari.modify(|r| r.ar24().set(1u8.into())) },
                    // SAFETY: AR25 is a RWH bit
                    25 => unsafe { txbari.modify(|r| r.ar25().set(1u8.into())) },
                    // SAFETY: AR26 is a RWH bit
                    26 => unsafe { txbari.modify(|r| r.ar26().set(1u8.into())) },
                    // SAFETY: AR27 is a RWH bit
                    27 => unsafe { txbari.modify(|r| r.ar27().set(1u8.into())) },
                    // SAFETY: AR28 is a RWH bit
                    28 => unsafe { txbari.modify(|r| r.ar28().set(1u8.into())) },
                    // SAFETY: AR29 is a RWH bit
                    29 => unsafe { txbari.modify(|r| r.ar29().set(1u8.into())) },
                    // SAFETY: AR30 is a RWH bit
                    30 => unsafe { txbari.modify(|r| r.ar30().set(1u8.into())) },
                    // SAFETY: AR31 is a RWH bit
                    31 => unsafe { txbari.modify(|r| r.ar31().set(1u8.into())) },
                    _ => {
                        // Invalid id, nothing to do
                    }
                }
            }

            // TODO The original code does not work with current PAC
            pub(crate) fn get_data_field_size(&self, from: ReadFrom) -> u8 {
                // SAFETY: each bit of RXESCI is at least R
                let rx_esc = unsafe { self.reg.rx().rxesci().read() };
                let size_code: u8 = match from {
                    ReadFrom::Buffer(_) => rx_esc.rbds().get().0,
                    ReadFrom::RxFifo0 => rx_esc.f0ds().get().0,
                    ReadFrom::RxFifo1 => rx_esc.f1ds().get().0,
                };

                if size_code < (DataFieldSize::_32 as u8) {
                    (size_code + 2) * 4
                } else {
                    (size_code - 3) * 16
                }
            }

            pub(crate) fn get_tx_buffer_data_field_size(&self) -> u8 {
                let size_code: u8 =
                // SAFETY: each bit of TXESCI is at least R
                    (unsafe { self.reg.tx().txesci().read() }.get_raw() & 0x2) as u8;
                if size_code < (DataFieldSize::_32 as u8) {
                    (size_code + 2) * 4
                } else {
                    (size_code - 3) * 16
                }
            }

            pub(crate) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
                // SAFETY: each bit of TXBRPI is RH
                let txbrpi = unsafe { self.reg.tx().txbrpi().read() };
                let id: u8 = tx_buffer_id.into();
                match id {
                    0 => txbrpi.trp0().get().0 == 1,
                    1 => txbrpi.trp1().get().0 == 1,
                    2 => txbrpi.trp2().get().0 == 1,
                    3 => txbrpi.trp3().get().0 == 1,
                    4 => txbrpi.trp4().get().0 == 1,
                    5 => txbrpi.trp5().get().0 == 1,
                    6 => txbrpi.trp6().get().0 == 1,
                    7 => txbrpi.trp7().get().0 == 1,
                    8 => txbrpi.trp8().get().0 == 1,
                    9 => txbrpi.trp9().get().0 == 1,
                    10 => txbrpi.trp10().get().0 == 1,
                    11 => txbrpi.trp11().get().0 == 1,
                    12 => txbrpi.trp12().get().0 == 1,
                    13 => txbrpi.trp13().get().0 == 1,
                    14 => txbrpi.trp14().get().0 == 1,
                    15 => txbrpi.trp15().get().0 == 1,
                    16 => txbrpi.trp16().get().0 == 1,
                    17 => txbrpi.trp17().get().0 == 1,
                    18 => txbrpi.trp18().get().0 == 1,
                    19 => txbrpi.trp19().get().0 == 1,
                    20 => txbrpi.trp20().get().0 == 1,
                    21 => txbrpi.trp21().get().0 == 1,
                    22 => txbrpi.trp22().get().0 == 1,
                    23 => txbrpi.trp23().get().0 == 1,
                    24 => txbrpi.trp24().get().0 == 1,
                    25 => txbrpi.trp25().get().0 == 1,
                    26 => txbrpi.trp26().get().0 == 1,
                    27 => txbrpi.trp27().get().0 == 1,
                    28 => txbrpi.trp28().get().0 == 1,
                    29 => txbrpi.trp29().get().0 == 1,
                    30 => txbrpi.trp30().get().0 == 1,
                    31 => txbrpi.trp31().get().0 == 1,
                    _ => false,
                }
            }
        }
    };
}

impl_can_node_effect!(pac::can0::N);
impl_can_node_effect!(pac::can1::N);
