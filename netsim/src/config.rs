use std::net::IpAddr;
use std::path;
use std::path::PathBuf;

use log::LevelFilter;
use serde::Deserialize;

use crate::units::{Bits, TimeInterval};

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    bootstrap_end_time: TimeInterval,
    data_directory: PathBuf,
    heartbeat_interval: TimeInterval,
    log_level: LevelFilter,
    parallelism: u64,
    seed: u64,
    stop_time: TimeInterval,
    template_directory: PathBuf,
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
    bandwidth_down: Bits,
    bandwidth_up: Bits,
    ip_addr: IpAddr,
    network_node_id: u64,
    options: HostDefaultsConfig,
    quantity: u64,
    processes: Vec<ProcessConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessConfig {
    args: String,
    environment: String,
    path: PathBuf,
    quantity: u64,
    start_time: TimeInterval,
    stop_time: TimeInterval,
}
