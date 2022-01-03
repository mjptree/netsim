use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use chrono::Duration;

use crate::host::Host;
use crate::task::Task;
use crate::time::SimulationTime;
use crate::worker::Worker;

pub struct TokenBucket {
    capacity: u64,
    remaining: u64,
    refill: u64,
}

impl TokenBucket {
    fn consume(&mut self, bytes: u64) {
        self.remaining.saturating_sub(bytes);
    }

    fn refill(&mut self) {
        self.remaining.saturating_add(self.refill);
    }

    const fn needs_refill(&self) -> bool {
        self.remaining < self.capacity
    }
}

pub struct Router;

pub struct Interface {
    ip_addr: IpAddr,
    send_bucket: TokenBucket,
    recv_bucket: TokenBucket,
    is_refill_pending: bool,
    upstream_router: Option<Router>,
    refill_started: SimulationTime,
}

impl Interface {
    fn receive_packets(&self, host: &Host) {}

    fn send_packets(&self, host: &Host) {}

    pub fn refill_buckets(&mut self, host: &Host) {
        self.is_refill_pending = false;
        self.send_bucket.refill();
        self.recv_bucket.refill();

        if self.upstream_router.is_some() {
            self.receive_packets(host);
        }

        self.send_packets(host);

        if self.is_refill_needed() {
            self.schedule_next_refill(host);
        }
    }

    const fn is_refill_needed(&self) -> bool {
        (self.send_bucket.needs_refill() || self.recv_bucket.needs_refill())
            && self.is_refill_pending
    }

    fn schedule_next_refill(this: Arc<Mutex<Self>>, host: &Host) {
        let now: SimulationTime = Worker::current_time()
            .expect("Current time not set for worker")
            .into();
        let interval = Self::refill_interval();
        let last_refill = now - this.lock().unwrap().refill_started;
        let next_refill = interval - last_refill % interval;
        let task = Task::RefillBuckets(this.clone());
        Worker::schedule_task(task, host, next_refill);
    }

    const fn refill_interval() -> SimulationTime {
        Duration::milliseconds(1).into()
    }
}

pub struct Packet;
