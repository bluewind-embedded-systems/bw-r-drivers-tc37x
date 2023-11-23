use crate::tracing::{LoadModifyStoreEntry, ReadEntry, ReportEntry, WriteEntry};
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter, Write};
use std::sync::{Arc, Mutex};
use tc37x_pac::tracing::TraceGuard;

struct ReadFifoEntry {
    addr: usize,
    len: usize,
    val: u64,
}

#[derive(Default)]
struct ReadFifo(VecDeque<ReadFifoEntry>);

#[derive(Default)]
struct SharedData {
    log: Log,
    read_fifo: ReadFifo,
}

struct Reporter {
    shared_data: Arc<Mutex<SharedData>>,
}

pub struct Report {
    shared_data: Arc<Mutex<SharedData>>,
    _guard: TraceGuard,
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn take_log(&self) -> Log {
        let mut g = self.shared_data.lock().unwrap();
        let len = g.log.0.len();
        Log(g.log.0.drain(0..len).collect())
    }

    pub fn expect_read(&self, addr: usize, len: usize, val: u64) {
        self.shared_data
            .lock()
            .unwrap()
            .read_fifo
            .0
            .push_front(ReadFifoEntry { addr, len, val })
    }

    pub fn all_reads_are_consumed(&self) -> bool {
        self.shared_data.lock().unwrap().read_fifo.0.is_empty()
    }
}

impl Reporter {
    fn push(&self, report: ReportEntry) {
        self.shared_data.lock().unwrap().log.0.push(report);
    }
}

impl tc37x_pac::tracing::Reporter for Reporter {
    fn read_volatile(&self, addr: usize, len: usize) -> u64 {
        self.push(ReportEntry::Read(ReadEntry { addr, len }));

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
            panic!("Unexpected read at address 0x{:08X} and len {}", addr, len)
        }
    }

    fn write_volatile(&self, addr: usize, len: usize, val: u64) {
        self.push(ReportEntry::Write(WriteEntry { addr, len, val }));
    }

    fn load_modify_store(&self, addr: usize, val: u64) {
        self.push(ReportEntry::LoadModifyStore(LoadModifyStoreEntry {
            addr,
            val,
        }));
    }
}

impl Drop for Reporter {
    fn drop(&mut self) {
        if !self.shared_data.lock().unwrap().read_fifo.0.is_empty() {
            panic!("More read where expected");
        }
    }
}

#[derive(Default)]
pub struct Log(Vec<ReportEntry>);

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            match entry {
                ReportEntry::Read(x) => {
                    write!(f, "r    0x{:08X} {:02}", x.addr, x.len);
                }
                ReportEntry::Write(x) => {
                    write!(f, "w    0x{:08X} {:02} 0x{:08X}", x.addr, x.len, x.val);
                }
                ReportEntry::LoadModifyStore(x) => {
                    let mask = (x.val >> 32);
                    let val = (x.val & 0xFFFFFFFF);
                    write!(f, "ldms 0x{:08X} 0x{:08X} 0x{:08X}", x.addr, mask, val);
                }
            }

            f.write_char('\n');
        }

        Ok(())
    }
}
