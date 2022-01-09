use core::cell;
use std::lazy;
use std::sync::{Arc, Mutex};

use crate::host::Host;
use crate::time::EmulatedTime;

#[derive(Clone, Copy)]
pub struct WorkerId(u32);

pub struct Process {
    id: u64,
}

pub struct Thread {
    id: u64,
}

struct Clock {
    now: Option<EmulatedTime>,
    last: Option<EmulatedTime>,
    barrier: Option<EmulatedTime>,
}

pub struct Worker {
    id: WorkerId,
    active_host: Option<Arc<Host>>,
    active_process: Option<Process>,
    active_thread: Option<Thread>,
    pool: Arc<Mutex<WorkerPool>>,
    clock: Clock,
    bootstrap_end_time: EmulatedTime,
}

std::thread_local! { static WORKER: lazy::OnceCell<cell::RefCell<Worker>> = lazy::OnceCell::new(); }

impl Worker {
    pub fn spawn(pool: Arc<Mutex<WorkerPool>>, id: WorkerId, bootstrap_end_time: EmulatedTime) {
        WORKER.with(|worker| {
            let _ = worker.set(cell::RefCell::new(Self {
                id,
                active_host: None,
                active_process: None,
                active_thread: None,
                pool,
                clock: Clock {
                    now: None,
                    last: None,
                    barrier: None,
                },
                bootstrap_end_time,
            }));
        })
    }

    fn with<F, R>(func: F) -> Option<R>
    where
        F: FnOnce(&Self) -> R,
    {
        WORKER
            .try_with(|worker| worker.get().map(|worker| func(&worker.borrow())))
            .ok()
            .flatten()
    }

    fn with_mut<F, R>(func: F) -> Option<R>
    where
        F: FnOnce(&mut Self) -> R,
    {
        WORKER
            .try_with(|worker| worker.get().map(|worker| func(&mut worker.borrow_mut())))
            .ok()
            .flatten()
    }

    pub fn set_active_host(host: Arc<Host>) {
        let _ = Self::with_mut(|worker| worker.active_host.replace(host))
            .expect("tried to set active host on uninitialized worker");
    }

    pub fn clear_active_host() {
        let _ = Self::with_mut(|worker| worker.active_host.take())
            .expect("tried to clear active host on uninitialized worker");
    }

    pub fn with_active_host<F, R>(func: F) -> Option<R>
    where
        F: FnOnce(&Arc<Host>) -> R,
    {
        Self::with(|worker| worker.active_host.as_ref().map(func)).flatten()
    }

    pub fn set_active_process(process: Process) {
        let _ = Self::with_mut(|worker| worker.active_process.replace(process))
            .expect("tried to set active process on uninitialized worker");
    }

    pub fn clear_active_process() {
        let _ = Self::with_mut(|worker| worker.active_process.take())
            .expect("tried to clear active process on uninitialized worker");
    }

    pub fn set_active_thread(thread: Thread) {
        let _ = Self::with_mut(|worker| worker.active_thread.replace(thread))
            .expect("tried to set active thread on uninitialized worker");
    }

    pub fn clear_active_thread() {
        let _ = Self::with_mut(|worker| worker.active_thread.take())
            .expect("tried to clear active thread on unitialized worker");
    }

    pub fn set_round_end_time(time: EmulatedTime) {
        let _ = Self::with_mut(|worker| worker.clock.barrier.replace(time))
            .expect("tried to set round time on uninitalized worker");
    }

    pub fn round_end_time() -> Option<EmulatedTime> {
        Self::with(|worker| worker.clock.barrier.clone()).flatten()
    }

    pub fn set_current_time(time: EmulatedTime) {
        let _ = Self::with_mut(|worker| worker.clock.now.replace(time))
            .expect("tried to set current time on unitialized worker");
    }

    pub fn clear_current_time() {
        let _ = Self::with_mut(|worker| worker.clock.now.take())
            .expect("tried to clear current time on uninitialized worker");
    }

    pub fn current_time() -> Option<EmulatedTime> {
        Self::with(|worker| worker.clock.now.clone()).flatten()
    }

    pub fn set_last_event_time(time: EmulatedTime) {
        let _ = Self::with_mut(|worker| worker.clock.last.replace(time))
            .expect("tried to set last event time on uninitalized worker");
    }

    pub fn is_alive() -> bool {
        Worker::with(|_| {}).is_some()
    }

    pub fn is_bootstrap_active() -> bool {
        Worker::with(|worker| {
            worker
                .clock
                .now
                .as_ref()
                .map_or(false, |now| *now < worker.bootstrap_end_time)
        })
        .unwrap_or_else(|| unreachable!())
    }

    // pub fn is_scheduler_running() -> bool {
    //     Worker::scheduler()
    //         .lock()
    //         .expect("tried to acquire poisoned manager lock")
    //         .is_running()
    // }

    pub fn worker_id() -> Option<WorkerId> {
        Worker::with(|worker| worker.id)
    }

    pub fn worker_pool() -> Arc<Mutex<WorkerPool>> {
        Worker::with_mut(|worker| worker.pool.clone())
            .expect("cannot access worker pool from unitialized worker")
    }

    // pub fn scheduler() -> Arc<Mutex<Scheduler>> {
    //     Worker::worker_pool()
    //         .lock()
    //         .expect("accessed scheduler through poisoned worker pool lock")
    //         .scheduler()
    // }

    // pub fn schedule_task(task: Task, host: &Host, delay: SimulationTime) -> bool {
    //     if !Self::is_scheduler_running() {
    //         return false;
    //     }

    //     if let Some(now) = Self::current_time().map(SimulationTime::from) {
    //         let event = Event::new(task, now + delay, host);
    //         Self::scheduler()
    //             .lock()
    //             .expect("tried to acquired poisoned scheduler lock")
    //             .push(event, host, delay)
    //     } else {
    //         false
    //     }
    // }
}

pub struct WorkerPool {}

impl WorkerPool {
    pub fn new() -> Self {
        Self {}
    }
}
