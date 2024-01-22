use core::marker::PhantomData;

use crate::can::baud_rate::{calculate_fast_bit_timing, BitTiming};
use crate::can::can_node::{
    FrameMode, Interrupt, InterruptGroup, InterruptLine, Priority, RxFifoMode, RxSel, Tos,
};
use crate::can::msg::{ReadFrom, RxBufferId, TxBufferId};
use crate::can::{DataFieldSize, Module, ModuleId, TxMode};
use crate::pac;
use tc37x_pac::can0::n::dbtpi::Tdc;
use tc37x_pac::can0::Can0;
use tc37x_pac::can1::Can1;
use tc37x_pac::hidden::RegValue;
use tc37x_pac::RegisterValue;

use super::NodeId;

pub(super) struct NodeEffects<T> {
    pub(crate) reg: T,
    pub(crate) node_id: NodeId,
}

const CCE_FALSE: tc37x_pac::can0::n::cccri::Cce = tc37x_pac::can0::n::cccri::Cce::CONST_00;
const CCE_TRUE: tc37x_pac::can0::n::cccri::Cce = tc37x_pac::can0::n::cccri::Cce::CONST_11;
const INIT_FALSE: tc37x_pac::can0::n::cccri::Init = tc37x_pac::can0::n::cccri::Init::CONST_00;
const INIT_TRUE: tc37x_pac::can0::n::cccri::Init = tc37x_pac::can0::n::cccri::Init::CONST_11;

impl NodeEffects<pac::can0::N> {
    pub(crate) fn new_node<T>(reg: pac::can0::N, node_id: NodeId) -> NodeEffects<pac::can0::N> {
        NodeEffects {
            reg,
            node_id,
        }
    }

    pub(crate) fn set_rx_fifo0_data_field_size(&self, size: u8) {
        use pac::can0::n::rx::rxesci::F0Ds;
        unsafe { self.reg.rx().rxesci().modify(|r| r.f0ds().set(F0Ds(size))) };
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

    pub(crate) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
        unsafe {
            self.reg.tx().txbtiei().modify(|mut r| {
                *r.data_mut_ref() |= 1 << tx_buffer_id.0;
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
                .modify(|r| r.tfqs().set(tc37x_pac::can0::n::tx::txbci::Tfqs(number)))
        };
    }

    // TODO Return a different type which implements methods needing configuration change enabled
    pub(crate) fn enable_configuration_change(&self) {
        let cccr = self.reg.cccri();

        if unsafe { cccr.read() }.init().get() == INIT_TRUE {
            unsafe { cccr.modify(|r| r.cce().set(CCE_FALSE)) };
            while !{ unsafe { cccr.read() }.cce().get() == CCE_FALSE } {}
            unsafe { cccr.modify(|r| r.init().set(INIT_FALSE)) };
            while !{ unsafe { cccr.read() }.init().get() == INIT_FALSE } {}
        }

        unsafe { cccr.modify(|r| r.init().set(INIT_TRUE)) };
        while !{ unsafe { cccr.read() }.init().get() == INIT_TRUE } {}

        unsafe { cccr.modify(|r| r.cce().set(CCE_TRUE).init().set(INIT_TRUE)) };
    }

    // TODO Return a different type which does not implement methods needing configuration change enabled
    pub(crate) fn disable_configuration_change(&self) {
        let cccr = self.reg.cccri();

        unsafe { cccr.modify(|r| r.cce().set(CCE_FALSE)) };
        while !{ unsafe { cccr.read() }.cce().get() == CCE_FALSE } {}

        unsafe { cccr.modify(|r| r.init().set(INIT_FALSE)) };
        while !{ unsafe { cccr.read() }.init().get() == INIT_FALSE } {}
    }

    pub(crate) fn set_nominal_bit_timing(&self, timing: &BitTiming) {
        unsafe {
            self.reg.nbtpi().modify(|r| {
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
            self.reg.dbtpi().modify(|r| {
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
        unsafe { self.reg.tx().txesci().modify(|r| r.tbds().set(tdbs.into())) };
    }

    pub(crate) fn set_tx_buffer_start_address(&self, address: u16) {
        unsafe { self.reg.tx().txbci().modify(|r| r.tbsa().set(address >> 2)) };
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
        unsafe { self.reg.dbtpi().modify(|r| r.tdc().set(Tdc::CONST_11)) };
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
        // TODO Check if "as u8" is necessary and safe
        unsafe { self.reg.npcri().modify(|r| r.rxsel().set(rx_sel as u8)) };
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

    pub(crate) fn get_rx_fifo0_get_index(&self) -> u8 {
        unsafe { self.reg.rx().rxf0si().read() }.f0gi().get()
    }

    pub(crate) fn get_rx_fifo1_get_index(&self) -> u8 {
        unsafe { self.reg.rx().rxf1si().read() }.f1gi().get()
    }

    pub(crate) fn set_rx_buffer_data_field_size(&self, size: u8) {
        unsafe { self.reg.rx().rxesci().modify(|r| r.rbds().set(size.into())) };
        todo!()
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
    pub(crate) fn set_tx_buffer_add_request(&self /*,tx_buffer_id: TxBufferId*/) {
        unsafe {
            self.reg
                .tx()
                .txbari()
                // TODO argument is now a postfix?
                .modify(|r| r.ar0(/*tx_buffer_id.into()*/).set(1u8.into()))
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
        let size_code: u8 = (unsafe { self.reg.tx().txesci().read() }.get_raw() & 0x2) as u8;
        if size_code < (DataFieldSize::_32 as u8) {
            (size_code + 2) * 4
        } else {
            (size_code - 3) * 16
        }
    }

    pub(crate) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
        match (tx_buffer_id.0) {
            0 => unsafe { self.reg.tx().txbrpi().read() }.trp0().get().0 == 1,
            1 => unsafe { self.reg.tx().txbrpi().read() }.trp1().get().0 == 1,
            2 => unsafe { self.reg.tx().txbrpi().read() }.trp2().get().0 == 1,
            3 => unsafe { self.reg.tx().txbrpi().read() }.trp3().get().0 == 1,
            4 => unsafe { self.reg.tx().txbrpi().read() }.trp4().get().0 == 1,
            5 => unsafe { self.reg.tx().txbrpi().read() }.trp5().get().0 == 1,
            6 => unsafe { self.reg.tx().txbrpi().read() }.trp6().get().0 == 1,
            7 => unsafe { self.reg.tx().txbrpi().read() }.trp7().get().0 == 1,
            8 => unsafe { self.reg.tx().txbrpi().read() }.trp8().get().0 == 1,
            // TODO implement trp0..trp31
            _ => todo!(),
        }
    }
}
