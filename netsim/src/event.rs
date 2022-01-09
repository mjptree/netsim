use std::sync::Arc;

use crate::host::Host;
use crate::task::Task;
use crate::time::SimulationTime;
use crate::worker::Worker;

#[derive(PartialEq, PartialOrd)]
pub struct EventId(i64);

pub struct Event {
    src: Arc<Host>,
    dst: Arc<Host>,
    task: Arc<Task>,
    time: SimulationTime,
    // event_id: EventId,
}

impl Event {
    pub fn new(task: Arc<Task>, delay: SimulationTime, src: Arc<Host>, dst: Arc<Host>) -> Self {
        Self {
            src,
            dst,
            task,
            time: delay,
            //event_id: src.new_event_id(),
        }
    }

    pub fn host(&self) -> Arc<Host> {
        self.dst.clone()
    }

    pub fn execute(&self) {
        Worker::set_active_host(self.dst.clone());

        // self.task.execute(self.host())
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other) == Some(core::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = self.time.partial_cmp(&other.time);

        if ord != Some(core::cmp::Ordering::Equal) {
            return ord;
        }

        let ord = self.dst.partial_cmp(&other.dst);

        if ord != Some(core::cmp::Ordering::Equal) {
            return ord;
        }

        let ord = self.src.partial_cmp(&other.src);

        if ord != Some(core::cmp::Ordering::Equal) {
            return ord;
        }
        ord
        //self.event_id.partial_cmp(&other.event_id)
    }
}
