use std::error;

pub struct Driver;

impl Driver {
    pub fn new() -> Self {
        Driver
    }

    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
