use crate::event::Event;
use crate::host::Host;
use crate::time::SimulationTime;

pub struct Scheduler;

impl Scheduler {
    pub fn push(&self, event: Event, host: &Host, delay: SimulationTime) -> bool {
        todo!()
    }
}
