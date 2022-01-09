use std::sync::{Arc, Mutex};

use crate::host::Host;
use crate::net::Interface;
use crate::time::SimulationTime;
use crate::worker::Worker;

pub enum Task {
    // Close(Box<dyn Fn(&Host)>),
    // Expire(Box<dyn Fn(&Host)>),
    // HeartBeat(Box<dyn Fn(&Host)>),
    RefillBuckets(Arc<Mutex<Interface>>),
    // Retransmit(Box<dyn Fn(&Host)>),
    // StartProcess(Arc<Process>),
    // StopProcess(Arc<Process>),
    // StartThread(Arc<Process>, Arc<Thread>),
    // ReceivePacket(Arc<Interface>, Arc<Packet>),
}

impl Task {
    pub fn execute(&mut self, host: Arc<Host>) {
        use Task::*;

        match self {
            // Close(func) => func(host),
            // Expire(func) => func(host),
            // HeartBeat(func) => func(host),
            // Retransmit(func) => func(host),
            RefillBuckets(ref mut interface) => {
                let mut this = interface.lock().unwrap();
                //this.refill_buckets(host);

                if (this.is_refill_needed()) {
                    let now: SimulationTime = Worker::current_time()
                        .expect("Current time not set for worker")
                        .into();
                    let interval = Interface::refill_interval();
                    let last_refill = now - this.refill_started();
                    let next_refill = interval - last_refill % interval;
                    let task = Task::RefillBuckets(interface.clone());
                    //Worker::schedule_task(task, host, next_refill);
                }
            }
            // StartProcess(process) => todo!(),
            // StopProcess(process) => todo!(),
            // StartThread(process, thread) => todo!(),
            // ReceivePacket(interface, packet) => todo!(),
        }
    }
}
