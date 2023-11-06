use crate::tracing::{ReportAction, ReportData};
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tc37x_pac::tracing::TraceGuard;

struct ReadFifoEntry {
    addr: usize,
    len: usize,
    val: u32,
}

#[derive(Default)]
struct ReadFifo(VecDeque<ReadFifoEntry>);

#[derive(Default)]
struct SharedData {
    log: Vec<ReportData>,
    read_fifo: ReadFifo,
}

struct Reporter {
    shared_data: Arc<Mutex<SharedData>>,
}

pub struct Report {
    shared_data: Arc<Mutex<SharedData>>,
    _guard: TraceGuard,
}

impl Report {
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(SharedData::default()));
        let reporter = Reporter {
            shared_data: data.clone(),
        };
        let guard = TraceGuard::new(reporter);
        Self {
            shared_data: data.clone(),
            _guard: guard,
        }
    }

    pub fn get_logs(&self) -> Vec<ReportData> {
        let mut g = self.shared_data.lock().unwrap();
        let len = g.log.len();
        g.log.drain(0..len).collect()
    }

    pub fn expect_read(&self, addr: usize, len: usize, val: u32) {
        self.shared_data
            .lock()
            .unwrap()
            .read_fifo
            .0
            .push_front(ReadFifoEntry { addr, len, val })
    }
}

impl Reporter {
    fn push(&self, report: ReportData) {
        self.shared_data.lock().unwrap().log.push(report);
    }

    fn report(&self, action: ReportAction, addr: usize, len: usize) {
        self.push(ReportData::new(action, addr, len))
    }
}

impl tc37x_pac::tracing::Reporter for Reporter {
    fn read_volatile(&self, addr: usize, len: usize) -> u32 {
        self.report(ReportAction::Read, addr, len);

        let entry = self
            .shared_data
            .lock()
            .unwrap()
            .read_fifo
            .0
            .pop_front()
            .expect("Unexpected read");

        if entry.addr == addr && entry.len == len {
            entry.val
        } else {
            panic!("Unexpected read at address {} with len {}", addr, len)
        }
    }

    fn write_volatile(&self, addr: usize, len: usize, val: u32) {
        self.report(ReportAction::Write(val), addr, len);
    }

    fn load_modify_store(&self, addr: usize, val: u64) {
        self.report(
            ReportAction::LoadModifyStore(val),
            addr,
            core::mem::size_of::<u64>(),
        );
    }
}

impl Drop for Reporter {
    fn drop(&mut self) {
        if !self.shared_data.lock().unwrap().read_fifo.0.is_empty() {
            panic!("More read where expected");
        }
    }
}
