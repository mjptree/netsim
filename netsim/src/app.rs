use std::error;
use std::io;
use std::string::ParseError;

use crate::cli::Args;
use crate::sim::Driver;

pub struct App;

impl App {
    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        Driver::new().run()
    }
}
pub struct AppBuilder;

impl AppBuilder {
    pub fn with_args(cli_args: Args) -> Result<Self, ParseError> {
        Ok(Self)
    }

    pub fn build(self) -> io::Result<App> {
        Ok(App)
    }
}
