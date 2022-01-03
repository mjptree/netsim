mod units;

use std::net;
use std::path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    bootstrap_end_time: units::TimeInterval,
    data_directory: path::PathBuf,
    heartbeat_interval: units::TimeInterval,
    log_level: log::LevelFilter,
    parallelism: u64,
    seed: u64,
    stop_time: units::TimeInterval,
    template_directory: path::PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    use_shortest_path: bool,
}

#[derive(Debug, Deserialize)]
pub struct HostDefaultsConfig {
    log_level: log::LevelFilter,
    pcap_directory: path::PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct HostsConfig {
    bandwidth_down: units::Bandwidth,
    bandwidth_up: units::Bandwidth,
    ip_addr: net::IpAddr,
    network_node_id: u64,
    options: HostDefaultsConfig,
    quantity: u64,
    processes: Vec<ProcessConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessConfig {
    args: String,
    environment: String,
    path: path::PathBuf,
    quantity: u64,
    start_time: units::TimeInterval,
    stop_time: units::TimeInterval,
}
