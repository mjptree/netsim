use std::env;
use std::error;

use netsim::app::AppBuilder;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = AppBuilder::with_args(env::args().collect())?.build()?;

    app.run()
}
