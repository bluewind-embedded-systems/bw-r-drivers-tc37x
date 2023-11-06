use crate::tracing::{ReportAction, ReportData};
use std::any::Any;
use std::sync::{Arc, Mutex};

pub struct Reporter(Arc<Mutex<Vec<ReportData>>>);

pub struct Report(Arc<Mutex<Vec<ReportData>>>);

pub fn reporter() -> (Reporter, Report) {
    let x = Arc::new(Mutex::new(Vec::new()));
    (Reporter(x.clone()), Report(x.clone()))
}

impl Report {
    pub fn get_logs(&self) -> Vec<ReportData> {
        let mut g = self.0.lock().unwrap();
        let len = g.len();
        g.drain(0..len).collect()
    }
}

impl Reporter {
    fn push(&self, report: ReportData) {
        self.0.lock().unwrap().push(report);
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
