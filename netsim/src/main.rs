use std::error;

use clap::Parser;

use netsim::app::AppBuilder;
use netsim::cli::Args;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = AppBuilder::with_args(Args::parse())?.build()?;

    app.run()
}
