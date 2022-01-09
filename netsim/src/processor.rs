use std::sync::Mutex;
use std::time::Duration;

use crate::time::PerfTimer;

pub struct Processor {
    cpu_id: u64,
    ready_workers: Vec<usize>,
    done_workers: Vec<usize>,
    idle_timer: Mutex<PerfTimer>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            cpu_id: 0,
            ready_workers: Vec::new(),
            done_workers: Vec::new(),
            idle_timer: Mutex::new(PerfTimer::start()),
        }
    }

    pub fn push_ready_worker(&mut self, worker: usize) {
        self.ready_workers.push(worker);
    }

    pub fn pop_ready_worker(&mut self) -> Option<usize> {
        self.ready_workers.pop()
    }

    pub fn push_done_worker(&mut self, worker: usize) {
        self.done_workers.push(worker);
    }

    pub fn finish_task(&mut self) {
        std::mem::swap(&mut self.ready_workers, &mut self.done_workers);
    }

    pub fn resume_idle_timer(&self) {
        self.idle_timer.lock().unwrap().resume();
    }

    pub fn pause_idle_timer(&self) {
        self.idle_timer.lock().unwrap().pause();
    }

    pub fn lapsed_idle_time(&self) -> Duration {
        self.idle_timer.lock().unwrap().lapsed()
    }
}

pub struct Processors {
    processors: Vec<Processor>,
}

impl Processors {
    pub fn new(size: usize) -> Self {
        Self {
            processors: (0..size).map(|_| Processor::new()).collect(),
        }
    }

    pub fn push_ready_worker(&mut self, processor: usize, worker: usize) {
        self.processors[processor].push_ready_worker(worker);
    }

    pub fn push_done_worker(&mut self, processor: usize, worker: usize) {
        self.processors[processor].push_done_worker(worker);
    }

    pub fn pop_worker_to_run_on(&mut self, processor: usize) -> Option<usize> {
        let worker = self.processors[processor..]
            .iter_mut()
            .map(|p| p.pop_ready_worker())
            .filter(Option::is_some)
            .next();

        worker
            .or_else(|| {
                self.processors[processor..]
                    .iter_mut()
                    .map(|p| p.pop_ready_worker())
                    .filter(Option::is_some)
                    .next()
            })
            .flatten()
    }

    pub fn finish_task(&mut self) {
        self.processors
            .iter_mut()
            .for_each(|processor| processor.finish_task());
    }

    pub fn cpu_id(&self, processor: usize) -> u64 {
        self.processors[processor].cpu_id
    }

    pub fn resume_idle_timer(&self, processor: usize) {
        self.processors[processor].resume_idle_timer();
    }

    pub fn pause_idle_timer(&self, processor: usize) {
        self.processors[processor].pause_idle_timer();
    }

    pub fn lapsed_idle_time(&self, processor: usize) -> Duration {
        self.processors[processor].lapsed_idle_time()
    }
}
