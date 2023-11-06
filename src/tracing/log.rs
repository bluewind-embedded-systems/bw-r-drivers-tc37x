use crate::tracing::{ReportAction, ReportData};
use std::any::Any;
use std::sync::{Arc, Mutex};
use tc37x_pac::tracing::TraceGuard;

struct Reporter {
    shared_log: Arc<Mutex<Vec<ReportData>>>,
}

pub struct Report {
    data: Arc<Mutex<Vec<ReportData>>>,
    _guard: TraceGuard,
}

impl Report {
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(Vec::new()));
        let reporter = Reporter {
            shared_log: data.clone(),
        };
        let guard = TraceGuard::new(reporter);
        Self {
            data: data.clone(),
            _guard: guard,
        }
    }

    pub fn get_logs(&self) -> Vec<ReportData> {
        let mut g = self.data.lock().unwrap();
        let len = g.len();
        g.drain(0..len).collect()
    }
}

impl Reporter {
    fn push(&self, report: ReportData) {
        self.shared_log.lock().unwrap().push(report);
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
