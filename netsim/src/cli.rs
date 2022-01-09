use std::path::PathBuf;

use log::LevelFilter;

use crate::units::TimeInterval;

#[derive(clap::Parser)]
#[clap(name = "NetSim", version = std::env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[clap(flatten)]
    general: GeneralOptions,
    #[clap(flatten)]
    network: NetworkOptions,
    #[clap(flatten)]
    host_default: HostDefaultOptions,
}

#[derive(clap::Args)]
#[clap(help_heading = "GENERAL")]
pub struct GeneralOptions {
    #[clap(short, long, value_name = "path")]
    output_directory: Option<PathBuf>,
    #[clap(short, long, value_name = "level")]
    log_level: Option<LevelFilter>,
    #[clap(long, value_name = "N")]
    seed: Option<u64>,
    #[clap(long, value_name = "seconds")]
    stop_time: Option<TimeInterval>,
    #[clap(long, value_name = "seconds")]
    bootstrap_end_time: Option<TimeInterval>,
    #[clap(long, value_name = "seconds")]
    heartbeat_interval: Option<TimeInterval>,
}

#[derive(clap::Args)]
#[clap(help_heading = "NETWORK")]
pub struct NetworkOptions {
    #[clap(long, value_name = "bool")]
    use_shortest_path: Option<bool>,
}

#[derive(clap::Args)]
#[clap(help_heading = "HOST DEFAULTS")]
pub struct HostDefaultOptions {
    #[clap(long = "host-log-level", name = "host-log-level", value_name = "level")]
    log_level: Option<LevelFilter>,
    #[clap(long, value_name = "path")]
    pcap: Option<PathBuf>,
}
