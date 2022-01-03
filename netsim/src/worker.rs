use core::cell;
use std::lazy;
use std::sync::{Arc, Mutex};

use crate::event::Event;
use crate::host::Host;
use crate::manager::Manager;
use crate::scheduler::{self, Scheduler};
use crate::task::Task;
use crate::time::{EmulatedTime, SimulationTime};

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
        F: FnOnce(&Worker) -> R,
    {
        WORKER
            .try_with(|worker| worker.get().map(|worker| func(&worker.borrow())))
            .ok()
            .flatten()
    }

    fn with_mut<F, R>(func: F) -> Option<R>
    where
        F: FnOnce(&mut Worker) -> R,
    {
        WORKER
            .try_with(|worker| worker.get().map(|worker| func(&mut worker.borrow_mut())))
            .ok()
            .flatten()
    }

    pub fn set_active_host(host: Arc<Host>) {
        let _ = Worker::with_mut(|worker| worker.active_host.replace(host)).unwrap();
    }

    pub fn clear_active_host() {
        let _ = Worker::with_mut(|worker| worker.active_host.take()).unwrap();
    }

    pub fn with_active_host<F, R>(func: F) -> Option<R>
    where
        F: FnOnce(&Arc<Host>) -> R,
    {
        Worker::with(|worker| worker.active_host.as_ref().map(func)).flatten()
    }

    pub fn set_active_process(process: Process) {
        let _ = Worker::with_mut(|worker| worker.active_process.replace(process)).unwrap();
    }

    pub fn clear_active_process() {
        let _ = Worker::with_mut(|worker| worker.active_process.take()).unwrap();
    }

    pub fn set_active_thread(thread: Thread) {
        let _ = Worker::with_mut(|worker| worker.active_thread.replace(thread)).unwrap();
    }

    pub fn clear_active_thread() {
        let _ = Worker::with_mut(|worker| worker.active_thread.take()).unwrap();
    }

    pub fn set_round_end_time(time: EmulatedTime) {
        let _ = Worker::with_mut(|worker| worker.clock.barrier.replace(time)).unwrap();
    }

    pub fn round_end_time() -> Option<EmulatedTime> {
        Worker::with(|worker| worker.clock.barrier).flatten()
    }

    pub fn set_current_time(time: EmulatedTime) {
        let _ = Worker::with_mut(|worker| worker.clock.now.replace(time)).unwrap();
    }

    pub fn clear_current_time() {
        let _ = Worker::with_mut(|worker| worker.clock.now.take());
    }

    pub fn current_time() -> Option<EmulatedTime> {
        Worker::with(|worker| worker.clock.now).flatten()
    }

    pub fn set_last_event_time(time: EmulatedTime) {
        let _ = Worker::with_mut(|worker| worker.clock.last.replace(time)).unwrap();
    }

    pub fn is_alive() -> bool {
        Worker::with(|_| {}).is_some()
    }

    pub fn is_bootstrap_active() -> bool {
        Worker::with(|worker| worker.clock.now.unwrap() < worker.bootstrap_end_time).unwrap()
    }

    pub fn is_scheduler_running() -> bool {
        Worker::manager().lock().unwrap().is_scheduler_running()
    }

    pub fn worker_id() -> Option<WorkerId> {
        Worker::with(|worker| worker.id)
    }

    pub fn worker_pool() -> Arc<Mutex<WorkerPool>> {
        Worker::with_mut(|worker| worker.pool.clone()).unwrap()
    }

    pub fn manager() -> Arc<Mutex<Manager>> {
        Worker::worker_pool().lock().unwrap().manager()
    }

    pub fn scheduler() -> Arc<Mutex<Scheduler>> {
        Worker::worker_pool().lock().unwrap().scheduler()
    }

    pub fn schedule_task(task: Task, host: &Host, delay: SimulationTime) -> bool {
        if !Self::is_scheduler_running() {
            return false;
        }

        if let Some(now) = Self::current_time().map(SimulationTime::from) {
            let event = Event::new(task, now + delay, host);
            Self::scheduler().lock().unwrap().push(event, host, delay)
        } else {
            false
        }
    }
}

pub struct WorkerPool {
    manager: Arc<Mutex<Manager>>,
    scheduler: Arc<Mutex<Scheduler>>,
}

impl WorkerPool {
    pub fn manager(&self) -> Arc<Mutex<Manager>> {
        self.manager.clone()
    }

    pub fn scheduler(&self) -> Arc<Mutex<Scheduler>> {
        self.scheduler.clone()
    }
}
