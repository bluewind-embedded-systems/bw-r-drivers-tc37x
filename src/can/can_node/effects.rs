use crate::can::baud_rate::{calculate_fast_bit_timing, BitTiming};
use crate::can::can_node::{
    FrameMode, Interrupt, InterruptGroup, InterruptLine, Priority, RxFifoMode, RxSel, Tos,
};
use crate::can::msg::{ReadFrom, RxBufferId};
use crate::can::{CanModuleId, DataFieldSize, TxBufferId, TxMode};
use core::intrinsics::transmute;
use tc37x_pac::can0::node::txesc::Tbds;
use tc37x_pac::hidden::RegValue;
use tc37x_pac::RegisterValue;

pub(super) struct NodeEffects {
    reg: tc37x_pac::can0::Node,
}

impl NodeEffects {
    pub(super) fn new(reg: tc37x_pac::can0::Node) -> Self {
        Self { reg }
    }

    pub(super) fn set_rx_fifo0_data_field_size(&self, size: DataFieldSize) {
        let size = tc37x_pac::can0::node::rxesc::F0Ds(size as u8);
        unsafe { self.reg.rxesc().modify(|r| r.f0ds().set(size)) };
    }

    pub(super) fn set_rx_fifo0_start_address(&self, address: u16) {
        unsafe { self.reg.rxf0c().modify(|r| r.f0sa().set(address >> 2)) };
    }

    pub(super) fn set_rx_fifo0_size(&self, size: u8) {
        unsafe { self.reg.rxf0c().modify(|r| r.f0s().set(size)) };
    }

    pub(super) fn set_rx_fifo0_watermark_level(&self, level: u8) {
        unsafe { self.reg.rxf0c().modify(|r| r.f0wm().set(level)) };
    }

    pub(super) fn set_rx_fifo0_operating_mode(&self, mode: RxFifoMode) {
        unsafe {
            self.reg
                .rxf0c()
                .modify(|r| r.f0om().set(mode == RxFifoMode::Overwrite))
        };
    }

    pub(super) fn enable_tx_buffer_transmission_interrupt(&self, tx_buffer_id: TxBufferId) {
        unsafe {
            self.reg.txbtie().modify(|mut r| {
                *r.data_mut_ref() |= 1 << tx_buffer_id.0;
                r
            })
        };
    }

    #[inline]
    pub(super) fn set_dedicated_tx_buffers_number(&self, number: u8) {
        unsafe { self.reg.txbc().modify(|r| r.ndtb().set(number)) };
    }

    #[inline]
    pub(super) fn set_tx_event_fifo_start_address(&self, address: u16) {
        unsafe { self.reg.txefc().modify(|r| r.efsa().set(address >> 2)) };
    }

    #[inline]
    pub(super) fn set_tx_event_fifo_size(&self, size: u8) {
        unsafe { self.reg.txefc().modify(|r| r.efs().set(size)) };
    }

    pub(super) fn set_transmit_fifo_queue_mode(&self, mode: TxMode) {
        let val = (mode as u8) != 0;
        unsafe { self.reg.txbc().modify(|r| r.tfqm().set(val)) };
    }

    pub(super) fn set_transmit_fifo_queue_size(&self, number: u8) {
        unsafe { self.reg.txbc().modify(|r| r.tfqs().set(number)) };
    }

    pub(super) fn enable_configuration_change(&self) {
        let cccr = self.reg.cccr();

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

    pub(super) fn disable_configuration_change(&self) {
        let cccr = self.reg.cccr();

        unsafe { cccr.modify(|r| r.cce().set(false)) };

        while unsafe { cccr.read() }.cce().get() {}

        unsafe { cccr.modify(|r| r.init().set(false)) };

        while unsafe { cccr.read() }.init().get() {}
    }

    pub(super) fn set_nominal_bit_timing(&self, timing: &BitTiming) {
        unsafe {
            self.reg.nbtp().modify(|r| {
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

    pub(super) fn set_data_bit_timing(&self, timing: &BitTiming) {
        // TODO Remove unwrap
        unsafe {
            self.reg.dbtp().modify(|r| {
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

    pub(super) fn set_tx_buffer_data_field_size(&self, tdbs: Tbds) {
        unsafe { self.reg.txesc().modify(|r| r.tbds().set(tdbs)) };
    }

    pub(super) fn set_tx_buffer_start_address(&self, address: u16) {
        unsafe { self.reg.txbc().modify(|r| r.tbsa().set(address >> 2)) };
    }

    pub(super) fn set_frame_mode(&self, fdoe: bool, brse: bool) {
        unsafe {
            self.reg
                .cccr()
                .modify(|r| r.fdoe().set(fdoe).brse().set(brse))
        };
    }

    pub(super) fn set_transceiver_delay_compensation_offset(&self, delay: u8) {
        unsafe { self.reg.dbtp().modify(|r| r.tdc().set(true)) };
        unsafe { self.reg.tdcr().modify(|r| r.tdco().set(delay)) };
    }

    pub(super) fn enable_interrupt(&self, interrupt: Interrupt) {
        unsafe {
            self.reg.ie().modify(|mut r| {
                *r.data_mut_ref() |= 1 << interrupt as u32;
                r
            })
        };
    }

    pub(super) fn set_interrupt_routing_group_1(&self, line: u32, group: u32) {
        unsafe {
            self.reg.grint1().modify(|mut r| {
                *r.data_mut_ref() |= line << group;
                r
            })
        };
    }

    pub(super) fn set_interrupt_routing_group_2(&self, line: u32, group: u32) {
        unsafe {
            self.reg.grint2().modify(|mut r| {
                *r.data_mut_ref() |= line << group;
                r
            })
        };
    }

    pub(super) fn connect_pin_rx(&self, rx_sel: RxSel) {
        // TODO Check if "as u8" is necessary and safe
        unsafe { self.reg.npcr().modify(|r| r.rxsel().set(rx_sel as u8)) };
    }

    pub(super) fn get_rx_fifo0_fill_level(&self) -> u8 {
        unsafe { self.reg.rxf0s().read() }.f0fl().get()
    }

    pub(super) fn get_rx_fifo1_fill_level(&self) -> u8 {
        unsafe { self.reg.rxf1s().read() }.f1fl().get()
    }

    pub(super) fn set_rx_buffers_start_address(&self, address: u16) {
        unsafe { self.reg.rxbc().modify(|r| r.rbsa().set(address >> 2)) };
    }

    pub(super) fn set_rx_fifo1_size(&self, size: u8) {
        unsafe { self.reg.rxf1c().modify(|r| r.f1s().set(size)) };
    }

    pub(super) fn set_rx_fifo1_start_address(&self, address: u16) {
        unsafe { self.reg.rxf1c().modify(|r| r.f1sa().set(address >> 2)) };
    }

    pub(super) fn set_rx_fifo1_watermark_level(&self, level: u8) {
        unsafe { self.reg.rxf1c().modify(|r| r.f1wm().set(level)) };
    }

    pub(super) fn is_tx_event_fifo_element_lost(&self) -> bool {
        unsafe { self.reg.txefs().read() }.tefl().get()
    }

    pub(super) fn is_tx_event_fifo_full(&self) -> bool {
        unsafe { self.reg.txefs().read() }.eff().get()
    }

    pub(super) fn is_tx_fifo_queue_full(&self) -> bool {
        unsafe { self.reg.txfqs().read() }.tfqf().get()
    }

    pub(super) fn pause_trasmission(&self, enable: bool) {
        unsafe { self.reg.cccr().modify(|r| r.txp().set(enable)) };
    }

    pub(super) fn set_standard_filter_list_start_address(&self, address: u16) {
        unsafe { self.reg.sidfc().modify(|r| r.flssa().set(address >> 2)) };
    }

    pub(super) fn set_standard_filter_list_size(&self, size: u8) {
        unsafe { self.reg.sidfc().modify(|r| r.lss().set(size)) };
    }

    pub(super) fn reject_remote_frames_with_standard_id(&self) {
        unsafe { self.reg.gfc().modify(|r| r.rrfs().set(true)) };
    }

    pub(super) fn set_extended_filter_list_start_address(&self, address: u16) {
        unsafe { self.reg.xidfc().modify(|r| r.flesa().set(address >> 2)) };
    }

    pub(super) fn set_extended_filter_list_size(&self, size: u8) {
        unsafe { self.reg.xidfc().modify(|r| r.lse().set(size)) };
    }

    pub(super) fn reject_remote_frames_with_extended_id(&self) {
        unsafe { self.reg.gfc().modify(|r| r.rrfe().set(true)) };
    }

    pub(super) fn get_tx_fifo_queue_put_index(&self) -> u8 {
        unsafe { self.reg.txfqs().read() }.tfqpi().get()
    }

    pub(super) fn get_rx_fifo0_get_index(&self) -> u8 {
        unsafe { self.reg.rxf0s().read() }.f0gi().get()
    }

    pub(super) fn get_rx_fifo1_get_index(&self) -> u8 {
        unsafe { self.reg.rxf1s().read() }.f1gi().get()
    }

    pub(super) fn set_rx_buffer_data_field_size(&self, _size: DataFieldSize) {
        // unsafe { self.reg.rxesc().modify(|r| r.rbds().set(size.into())) };
        todo!()
    }

    pub(super) fn is_rx_buffer_new_data_updated(&self, rx_buffer_id: u8) -> bool {
        let (data, mask) = if rx_buffer_id < 32 {
            let data = unsafe { self.reg.ndat1().read() }.data();
            let mask = 1 << u8::from(rx_buffer_id);
            (data, mask)
        } else {
            let data = unsafe { self.reg.ndat2().read() }.data();
            let mask = 1 << (u8::from(rx_buffer_id) - 32);
            (data, mask)
        };
        (data & mask) != 0
    }

    #[inline]
    pub(super) fn set_rx_fifo0_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
        unsafe {
            self.reg
                .rxf0a()
                .modify(|r| r.f0ai().set(rx_buffer_id.into()))
        };
    }

    #[inline]
    pub(super) fn set_rx_fifo1_acknowledge_index(&self, rx_buffer_id: RxBufferId) {
        unsafe {
            self.reg
                .rxf1a()
                .modify(|r| r.f1ai().set(rx_buffer_id.into()))
        };
    }

    #[inline]
    pub(super) fn is_tx_buffer_transmission_occured(&self, tx_buffer_id: u8) -> bool {
        let data = unsafe { self.reg.txbto().read() }.data();
        let mask = 1u32 << u32::from(tx_buffer_id);
        (data & mask) != 0
    }

    #[inline]
    pub(super) fn set_tx_buffer_add_request(&self, tx_buffer_id: TxBufferId) {
        unsafe {
            self.reg
                .txbar()
                .modify(|r| r.ar(tx_buffer_id.into()).set(true))
        }
    }

    pub(super) fn get_data_field_size(&self, _from: ReadFrom) -> u8 {
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

    pub(super) fn get_tx_buffer_data_field_size(&self) -> u8 {
        let size_code: u8 = (unsafe { self.reg.txesc().read() }.get_raw() & 0x2) as u8;
        if size_code < Tbds::TBDS_BUFFERSIZE32.0 {
            (size_code + 2) * 4
        } else {
            (size_code - 3) * 16
        }
    }

    pub(super) fn is_tx_buffer_request_pending(&self, tx_buffer_id: TxBufferId) -> bool {
        unsafe { self.reg.txbrp().read() }
            .trp(tx_buffer_id.into())
            .get()
    }
}
