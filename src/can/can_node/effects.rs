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

            pub(crate) fn set_rx_buffer_data_field_size(&self, size: u8) {
                unsafe { self.reg.rx().rxesci().modify(|r| r.rbds().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo0_data_field_size(&self, size: u8) {
                unsafe { self.reg.rx().rxesci().modify(|r| r.f0ds().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo1_data_field_size(&self, size: u8) {
                unsafe { self.reg.rx().rxesci().modify(|r| r.f1ds().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo0_start_address(&self, address: u16) {
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ci()
                        .modify(|r| r.f0sa().set(address >> 2))
                };
            }

            pub(crate) fn set_rx_fifo0_size(&self, size: u8) {
                unsafe { self.reg.rx().rxf0ci().modify(|r| r.f0s().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo0_watermark_level(&self, level: u8) {
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
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1om().set(overwrite.into()))
                };
            }

            pub(crate) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
                let id: u8 = tx_buffer_id.into();
                unsafe {
                    self.reg.tx().txbtiei().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << id;
                        r
                    })
                };
            }

            #[inline]
            pub(crate) fn set_dedicated_tx_buffers_number(&self, number: u8) {
                unsafe {
                    self.reg
                        .tx()
                        .txbci()
                        .modify(|r| r.ndtb().set(number.into()))
                };
            }

            #[inline]
            pub(crate) fn set_tx_event_fifo_start_address(&self, address: u16) {
                unsafe {
                    self.reg
                        .tx()
                        .txefci()
                        .modify(|r| r.efsa().set(address >> 2))
                };
            }

            #[inline]
            pub(crate) fn set_tx_event_fifo_size(&self, size: u8) {
                unsafe { self.reg.tx().txefci().modify(|r| r.efs().set(size.into())) };
            }

            // TODO Move logic to the caller
            pub(crate) fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
                let val = mode != TxMode::DedicatedBuffers;
                let val = u8::from(val);
                unsafe { self.reg.tx().txbci().modify(|r| r.tfqm().set(val.into())) };
            }

            pub(crate) fn set_transmit_fifo_queue_size(&self, number: u8) {
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

                if unsafe { cccr.read() }.init().get().0 == 1u8 {
                    unsafe { cccr.modify(|r| r.cce().set(0u8.into())) };
                    while !{ unsafe { cccr.read() }.cce().get().0 == 0u8 } {}
                    unsafe { cccr.modify(|r| r.init().set(0u8.into())) };
                    while !{ unsafe { cccr.read() }.init().get().0 == 0u8 } {}
                }

                unsafe { cccr.modify(|r| r.init().set(1u8.into())) };
                while !{ unsafe { cccr.read() }.init().get().0 == 1u8 } {}

                unsafe { cccr.modify(|r| r.cce().set(1u8.into()).init().set(1u8.into())) };
            }

            // TODO Return a different type which does not implement methods needing configuration change enabled
            pub(crate) fn disable_configuration_change(&self) {
                let cccr = self.reg.cccri();

                unsafe { cccr.modify(|r| r.cce().set(0u8.into())) };
                while !{ unsafe { cccr.read() }.cce().get().0 == 0u8 } {}

                unsafe { cccr.modify(|r| r.init().set(0u8.into())) };
                while !{ unsafe { cccr.read() }.init().get().0 == 0u8 } {}
            }

            pub(crate) fn set_nominal_bit_timing(&self, timing: &NominalBitTiming) {
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
                unsafe { self.reg.tx().txesci().modify(|r| r.tbds().set(tdbs.into())) };
            }

            pub(crate) fn set_tx_buffer_start_address(&self, address: u16) {
                unsafe { self.reg.tx().txbci().modify(|r| r.tbsa().set(address >> 2)) };
            }

            pub(crate) fn set_rx_buffer_start_address(&self, address: u16) {
                unsafe { self.reg.rx().rxbci().modify(|r| r.rbsa().set(address >> 2)) };
            }

            pub(crate) fn set_frame_mode(&self, fdoe: bool, brse: bool) {
                let fdoe = u8::from(fdoe);
                let brse = u8::from(brse);
                unsafe {
                    self.reg
                        .cccri()
                        .modify(|r| r.fdoe().set(fdoe.into()).brse().set(brse.into()))
                };
            }

            pub(crate) fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
                unsafe { self.reg.dbtpi().modify(|r| r.tdc().set(1u8.into())) };
                unsafe { self.reg.tdcri().modify(|r| r.tdco().set(delay)) };
            }

            pub(crate) fn enable_interrupt(&self, interrupt: Interrupt) {
                unsafe {
                    self.reg.iei().modify(|mut r| {
                        *r.data_mut_ref() |= 1 << interrupt as u32;
                        r
                    })
                };
            }

            #[inline]
            pub(crate) fn clear_interrupt_flag(&self, interrupt: Interrupt) {
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
                unsafe { self.reg.npcri().modify(|r| r.rxsel().set(rx_sel.into())) };
            }

            pub(crate) fn get_rx_fifo0_fill_level(&self) -> u8 {
                unsafe { self.reg.rx().rxf0si().read() }.f0fl().get()
            }

            pub(crate) fn get_rx_fifo1_fill_level(&self) -> u8 {
                unsafe { self.reg.rx().rxf1si().read() }.f1fl().get()
            }

            pub(crate) fn set_rx_buffers_start_address(&self, address: u16) {
                unsafe { self.reg.rx().rxbci().modify(|r| r.rbsa().set(address >> 2)) };
            }

            pub(crate) fn set_rx_fifo1_size(&self, size: u8) {
                unsafe { self.reg.rx().rxf1ci().modify(|r| r.f1s().set(size.into())) };
            }

            pub(crate) fn set_rx_fifo1_start_address(&self, address: u16) {
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1sa().set(address >> 2))
                };
            }

            pub(crate) fn set_rx_fifo1_watermark_level(&self, level: u8) {
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ci()
                        .modify(|r| r.f1wm().set(level.into()))
                };
            }

            pub(crate) fn is_tx_event_fifo_element_lost(&self) -> bool {
                unsafe { self.reg.tx().txefsi().read() }.tefl().get().0 == 1
            }

            pub(crate) fn is_tx_event_fifo_full(&self) -> bool {
                unsafe { self.reg.tx().txefsi().read() }.eff().get().0 == 1
            }

            pub(crate) fn is_tx_fifo_queue_full(&self) -> bool {
                unsafe { self.reg.tx().txfqsi().read() }.tfqf().get().0 == 1
            }

            pub(crate) fn pause_trasmission(&self, enable: bool) {
                unsafe {
                    self.reg
                        .cccri()
                        .modify(|r| r.txp().set(u8::from(enable).into()))
                };
            }

            pub(crate) fn set_standard_filter_list_start_address(&self, address: u16) {
                unsafe { self.reg.sidfci().modify(|r| r.flssa().set(address >> 2)) };
            }

            pub(crate) fn set_standard_filter_list_size(&self, size: u8) {
                unsafe { self.reg.sidfci().modify(|r| r.lss().set(size.into())) };
            }

            pub(crate) fn reject_remote_frames_with_standard_id(&self) {
                unsafe {
                    self.reg
                        .gfci()
                        .modify(|r| r.rrfs().set(u8::from(true).into()))
                };
            }

            pub(crate) fn set_extended_filter_list_start_address(&self, address: u16) {
                unsafe { self.reg.xidfci().modify(|r| r.flesa().set(address >> 2)) };
            }

            pub(crate) fn set_extended_filter_list_size(&self, size: u8) {
                unsafe {
                    self.reg
                        .xidfci()
                        .modify(|r| r.lse().set(u8::from(size).into()))
                };
            }

            pub(crate) fn reject_remote_frames_with_extended_id(&self) {
                unsafe { self.reg.gfci().modify(|r| r.rrfe().set(1u8.into())) };
            }

            pub(crate) fn get_tx_fifo_queue_put_index(&self) -> u8 {
                unsafe { self.reg.tx().txfqsi().read() }.tfqpi().get()
            }

            pub(crate) fn get_rx_fifo0_get_index(&self) -> RxBufferId {
                let idx: u8 = unsafe { self.reg.rx().rxf0si().read() }.f0gi().get();
                // SAFETY: idx is always between 0 and 63
                unsafe { RxBufferId::new_unchecked(idx) }
            }

            pub(crate) fn get_rx_fifo1_get_index(&self) -> RxBufferId {
                let idx: u8 = unsafe { self.reg.rx().rxf1si().read() }.f1gi().get();
                // SAFETY: idx is always between 0 and 63
                unsafe { RxBufferId::new_unchecked(idx) }
            }

            pub(crate) fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: u8) -> bool {
                let (data, mask) = if rx_buffer_id < 32 {
                    // last number value in the reg name is the node id
                    let data = unsafe { self.reg.ndat1i().read() }.data();
                    let mask = 1 << u8::from(rx_buffer_id);
                    (data, mask)
                } else {
                    // last number value in the reg name is the node id
                    let data = unsafe { self.reg.ndat2i().read() }.data();
                    let mask = 1 << (u8::from(rx_buffer_id) - 32);
                    (data, mask)
                };
                (data & mask) != 0
            }

            #[inline]
            pub(crate) fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                unsafe {
                    self.reg
                        .rx()
                        .rxf0ai()
                        .modify(|r| r.f0ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
            pub(crate) fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
                unsafe {
                    self.reg
                        .rx()
                        .rxf1ai()
                        .modify(|r| r.f1ai().set(rx_buffer_id.into()))
                };
            }

            #[inline]
            pub(crate) fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: u8) -> bool {
                let data = unsafe { self.reg.tx().txbtoi().read() }.data();
                let mask = 1u32 << u32::from(tx_buffer_id);
                (data & mask) != 0
            }

            #[inline]
            pub(crate) fn set_tx_buffer_add_request(&self, id: u8) {
                let txbari = self.reg.tx().txbari();
                match id {
                    0 => unsafe { txbari.modify(|r| r.ar0().set(1u8.into())) },
                    1 => unsafe { txbari.modify(|r| r.ar1().set(1u8.into())) },
                    2 => unsafe { txbari.modify(|r| r.ar2().set(1u8.into())) },
                    3 => unsafe { txbari.modify(|r| r.ar3().set(1u8.into())) },
                    4 => unsafe { txbari.modify(|r| r.ar4().set(1u8.into())) },
                    5 => unsafe { txbari.modify(|r| r.ar5().set(1u8.into())) },
                    6 => unsafe { txbari.modify(|r| r.ar6().set(1u8.into())) },
                    7 => unsafe { txbari.modify(|r| r.ar7().set(1u8.into())) },
                    8 => unsafe { txbari.modify(|r| r.ar8().set(1u8.into())) },
                    9 => unsafe { txbari.modify(|r| r.ar9().set(1u8.into())) },
                    10 => unsafe { txbari.modify(|r| r.ar10().set(1u8.into())) },
                    11 => unsafe { txbari.modify(|r| r.ar11().set(1u8.into())) },
                    12 => unsafe { txbari.modify(|r| r.ar12().set(1u8.into())) },
                    14 => unsafe { txbari.modify(|r| r.ar14().set(1u8.into())) },
                    13 => unsafe { txbari.modify(|r| r.ar13().set(1u8.into())) },
                    15 => unsafe { txbari.modify(|r| r.ar15().set(1u8.into())) },
                    16 => unsafe { txbari.modify(|r| r.ar16().set(1u8.into())) },
                    17 => unsafe { txbari.modify(|r| r.ar17().set(1u8.into())) },
                    18 => unsafe { txbari.modify(|r| r.ar18().set(1u8.into())) },
                    19 => unsafe { txbari.modify(|r| r.ar19().set(1u8.into())) },
                    20 => unsafe { txbari.modify(|r| r.ar20().set(1u8.into())) },
                    21 => unsafe { txbari.modify(|r| r.ar21().set(1u8.into())) },
                    22 => unsafe { txbari.modify(|r| r.ar22().set(1u8.into())) },
                    23 => unsafe { txbari.modify(|r| r.ar23().set(1u8.into())) },
                    24 => unsafe { txbari.modify(|r| r.ar24().set(1u8.into())) },
                    25 => unsafe { txbari.modify(|r| r.ar25().set(1u8.into())) },
                    26 => unsafe { txbari.modify(|r| r.ar26().set(1u8.into())) },
                    27 => unsafe { txbari.modify(|r| r.ar27().set(1u8.into())) },
                    28 => unsafe { txbari.modify(|r| r.ar28().set(1u8.into())) },
                    29 => unsafe { txbari.modify(|r| r.ar29().set(1u8.into())) },
                    30 => unsafe { txbari.modify(|r| r.ar30().set(1u8.into())) },
                    31 => unsafe { txbari.modify(|r| r.ar31().set(1u8.into())) },
                    _ => {
                        // Invalid id, nothing to do
                    }
                }
            }

            // TODO The original code does not work with current PAC
            pub(crate) fn get_data_field_size(&self, from: ReadFrom) -> u8 {
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
                    (unsafe { self.reg.tx().txesci().read() }.get_raw() & 0x2) as u8;
                if size_code < (DataFieldSize::_32 as u8) {
                    (size_code + 2) * 4
                } else {
                    (size_code - 3) * 16
                }
            }

            pub(crate) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
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
