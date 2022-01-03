use std::sync::{Arc, Mutex};

use crate::host::Host;
use crate::net::{Interface, Packet};
use crate::worker::{Process, Thread};

pub enum Task {
    Close(Box<dyn Fn(&Host)>),
    Expire(Box<dyn Fn(&Host)>),
    HeartBeat(Box<dyn Fn(&Host)>),
    RefillBuckets(Arc<Mutex<Interface>>),
    Retransmit(Box<dyn Fn(&Host)>),
    StartProcess(Arc<Process>),
    StopProcess(Arc<Process>),
    StartThread(Arc<Process>, Arc<Thread>),
    ReceivePacket(Arc<Interface>, Arc<Packet>),
}

impl Task {
    pub fn execute(&mut self, host: &Host) {
        use Task::*;

        match self {
            Close(func) => func(host),
            Expire(func) => func(host),
            HeartBeat(func) => func(host),
            Retransmit(func) => func(host),
            RefillBuckets(ref mut interface) => interface.lock().unwrap().refill_buckets(host),
            StartProcess(process) => todo!(),
            StopProcess(process) => todo!(),
            StartThread(process, thread) => todo!(),
            ReceivePacket(interface, packet) => todo!(),
        }
    }
}
