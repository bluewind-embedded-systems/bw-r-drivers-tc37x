use crate::tracing::{ReportAction, ReportData};
use std::any::Any;
use std::sync::{Arc, Mutex};
use tc37x_pac::tracing::TraceGuard;

#[derive(Default)]
struct SharedData {
    log: Vec<ReportData>,
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
        0 // FIXME
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
