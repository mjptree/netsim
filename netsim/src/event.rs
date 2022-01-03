use crate::host::Host;
use crate::task::Task;
use crate::time::SimulationTime;

pub struct Event {}

impl Event {
    pub fn new(task: Task, delay: SimulationTime, host: &Host) -> Self {
        Self {}
    }
}
