use std::collections::HashMap;
use std::error;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, PoisonError};

use rand::prelude::SmallRng;
use rand::{RngCore, SeedableRng};

use crate::event::Event;
use crate::host::{Host, HostId};
use crate::time::SimulationTime;
use crate::worker::WorkerPool;

type Seed = [u8; 32];

trait Policy {
    fn add_host(&mut self, host: &Host);
    fn hosts(&self) -> &Host;
    fn push(&self, event: &Event, host: &Host, time: SimulationTime);
    fn pop(&self, time: SimulationTime) -> Event;
    fn next_time(&self) -> SimulationTime;
}

struct HostSinglePolicy;

impl Policy for HostSinglePolicy {
    fn add_host(&mut self, host: &Host) {
        todo!()
    }

    fn hosts(&self) -> &Host {
        todo!()
    }

    fn push(&self, event: &Event, host: &Host, time: SimulationTime) {
        todo!()
    }

    fn pop(&self, time: SimulationTime) -> Event {
        todo!()
    }

    fn next_time(&self) -> SimulationTime {
        todo!()
    }
}

pub struct Scheduler {
    is_running: bool,
    worker: WorkerPool,
    hosts: HashMap<HostId, Host>,
    //policy: Box<dyn Policy>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            is_running: false,
            worker: WorkerPool::new(),
            hosts: HashMap::new(),
            //policy: Box::new(),
        }
    }

    fn start(&mut self) {
        self.is_running = true;
    }

    fn stop(&mut self) {
        self.is_running = false
    }

    pub fn push(&self, event: Event, host: &Host, delay: SimulationTime) -> bool {
        todo!()
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

pub struct Simulation {
    scheduler: Arc<Mutex<Scheduler>>,
}

impl Simulation {
    fn new() -> Self {
        Self {
            scheduler: Arc::new(Mutex::new(Scheduler::new())),
        }
    }

    fn run(&self) -> Result<(), Box<dyn error::Error>> {
        self.start_scheduler()?;

        loop {
            break;
        }

        self.stop_scheduler()?;
        Ok(())
    }

    fn start_scheduler(&self) -> Result<(), PoisonError<()>> {
        self.scheduler
            .lock()
            .map(|mut s| s.start())
            .map_err(|_| PoisonError::new(()))
    }

    fn stop_scheduler(&self) -> Result<(), PoisonError<()>> {
        self.scheduler
            .lock()
            .map(|mut s| s.stop())
            .map_err(|_| PoisonError::new(()))
    }
}

pub struct Driver {
    minimal_time_jump: SimulationTime,
    simulation: Simulation,
    random: Box<dyn RngCore>,
}

impl Driver {
    pub fn new() -> Self {
        Self {
            minimal_time_jump: SimulationTime::from_millis(10),
            simulation: Simulation::new(),
            random: Box::new(SmallRng::from_entropy()),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn error::Error>> {
        // load topology

        let seed = self.derive_seed();
        SmallRng::from_seed(seed);
        // create new simulation
        Ok(())
    }

    fn lapsed(&self) {}

    fn has_finished_current_rount(&self, simulation: &Simulation) -> bool {
        todo!()
    }

    fn latency_between(&self, src: &IpAddr, dst: &IpAddr) {}

    fn derive_seed(&mut self) -> Seed {
        let mut seed = [0u8; 32];
        self.random.fill_bytes(&mut seed);
        seed
    }
}
