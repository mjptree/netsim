use std::{net::IpAddr, sync::Arc};

use crate::task::Task;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HostId(isize);

impl From<isize> for HostId {
    fn from(value: isize) -> Self {
        HostId(value)
    }
}

impl From<HostId> for isize {
    fn from(value: HostId) -> Self {
        value.0
    }
}

pub struct HostParams {}

pub struct HostInfo {
    id: HostId,
    name: String,
    ip: IpAddr,
}

pub struct Host {
    info: Arc<HostInfo>,
}

impl Host {
    fn execute(&self, task: Task) {}

    pub fn info(&self) -> Arc<HostInfo> {
        self.info.clone()
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn id(&self) -> HostId {
        self.info.id
    }

    pub fn ip(&self) -> IpAddr {
        self.info.ip
    }
}

impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other) == Some(core::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Host {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}
