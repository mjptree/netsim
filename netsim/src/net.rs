use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::net::IpAddr;
use std::sync::Arc;

use crate::host::Host;
use crate::time::SimulationTime;
use crate::worker::Worker;

mod ipv4 {

    use std::net::Ipv4Addr;

    const fn addr(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr::new(a, b, c, d)
    }

    fn in_range(ip: Ipv4Addr, lower: Ipv4Addr, upper: Ipv4Addr) -> bool {
        ip >= lower && ip <= upper
    }

    fn is_current_network(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(0, 0, 0, 0), addr(0, 255, 255, 255))
    }
    fn is_private(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(10, 0, 0, 0), addr(10, 255, 255, 255))
            || in_range(ip, addr(172, 16, 0, 0), addr(172, 31, 255, 255))
            || in_range(ip, addr(192, 168, 0, 0), addr(192, 168, 255, 255))
    }

    fn is_shared(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(100, 64, 0, 0), addr(100, 127, 255, 255))
    }

    fn is_link_local(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(169, 254, 0, 0), addr(169, 254, 255, 255))
    }

    fn is_ietf_protocol_assigned(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(192, 0, 0, 0), addr(192, 0, 0, 255))
    }

    fn is_test_net(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(192, 0, 2, 0), addr(192, 0, 2, 255))
            || in_range(ip, addr(198, 51, 100, 0), addr(198, 51, 100, 255))
            || in_range(ip, addr(203, 0, 113, 0), addr(203, 0, 113, 255))
            || in_range(ip, addr(233, 252, 0, 0), addr(233, 252, 0, 255))
    }

    fn is_multicast(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(224, 0, 0, 0), addr(239, 255, 255, 255))
    }

    fn is_reserved(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(192, 88, 99, 0), addr(192, 88, 99, 255))
            || in_range(ip, addr(240, 0, 0, 0), addr(255, 255, 255, 255))
    }

    fn is_benchmark(ip: Ipv4Addr) -> bool {
        in_range(ip, addr(198, 18, 0, 0), addr(198, 19, 255, 255))
    }

    fn is_broadcast(ip: Ipv4Addr) -> bool {
        ip == addr(255, 255, 255, 255)
    }

    pub(super) fn is_restricted(ip: Ipv4Addr) -> bool {
        is_current_network(ip)
            || is_private(ip)
            || is_shared(ip)
            || is_link_local(ip)
            || is_ietf_protocol_assigned(ip)
            || is_test_net(ip)
            || is_reserved(ip)
            || is_broadcast(ip)
    }
}

// mod ipv6 {
//     use std::net::Ipv6Addr;

//     fn addr(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
//         Ipv6Addr::new(a, b, c, d, e, f, g, h)
//     }

//     fn in_range(ip: Ipv6Addr, lower: Ipv6Addr, upper: Ipv6Addr) -> bool {
//         ip >= lower && ip <= upper
//     }

//     fn is_unspecified(ip: Ipv6Addr) -> bool {
//         ip == Ipv6Addr::new(0, 0, 0 , 0, 0, 0, 0, 0)
//     }

//     fn is_ipv4_mapped(ip: Ipv6Addr) -> bool {
//         in_range(ip, addr(0, 0, 0, 0, 0, 0xffff, 0, 0), addr(0, 0, 0, 0, 0, 0xffff, 0xffff, 0xffff))
//     }

//     fn is_ipv4_translated(ip: Ipv6Addr) -> bool {
//         in_range(ip, addr(0, 0, 0, 0, 0xffff, 0, 0, 0), addr(0, 0, 0, 0, 0xffff, 0, 0xffff, 0xffff))
//     }

//     pub(super) fn is_restricted(ip: Ipv6Addr) -> bool {
//         ip.is_unspecified()
//             || ip.is
//     }

// }

fn is_restricted(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => ipv4::is_restricted(addr),
        IpAddr::V6(_) => false,
    }
}

pub struct TokenBucket {
    capacity: u64,
    remaining: u64,
    refill: u64,
}

impl TokenBucket {
    fn consume(&mut self, bytes: u64) {
        self.remaining.saturating_sub(bytes);
    }

    fn refill(&mut self) {
        self.remaining.saturating_add(self.refill);
    }

    const fn needs_refill(&self) -> bool {
        self.remaining < self.capacity
    }
}

pub struct Router {
    queue: VecDeque<Packet>,
}

impl Router {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn forward(&self, src: Host, packet: Packet) {
        todo!()
    }

    fn enqueue(&mut self) {
        todo!()
    }

    fn dequeue(&mut self) {
        todo!()
    }
}

pub struct Interface {
    ip_addr: IpAddr,
    send_bucket: TokenBucket,
    recv_bucket: TokenBucket,
    is_refill_pending: bool,
    upstream_router: Option<Router>,
    refill_started: SimulationTime,
}

impl Interface {
    fn receive_packets(&self, host: &Host) {}

    fn send_packets(&self, host: &Host) {}

    pub fn refill_buckets(&mut self, host: &Host) {
        self.is_refill_pending = false;
        self.send_bucket.refill();
        self.recv_bucket.refill();

        if self.upstream_router.is_some() {
            self.receive_packets(host);
        }

        self.send_packets(host);
    }

    pub const fn is_refill_needed(&self) -> bool {
        (self.send_bucket.needs_refill() || self.recv_bucket.needs_refill())
            && self.is_refill_pending
    }

    pub const fn refill_started(&self) -> SimulationTime {
        self.refill_started
    }

    pub fn refill_interval() -> SimulationTime {
        SimulationTime::from_millis(1)
    }
}

pub struct NameRecord {
    a: IpAddr,
    cname: String,
}

impl NameRecord {
    fn a(&self) -> IpAddr {
        self.a
    }

    fn cname(&self) -> String {
        self.cname.clone()
    }
}

pub struct NameServer {
    by_a: HashMap<IpAddr, Arc<NameRecord>>,
    by_cname: HashMap<String, Arc<NameRecord>>,
}

impl NameServer {
    fn lookup(&self, name: &str) -> Option<Arc<NameRecord>> {
        self.by_cname.get(name).cloned()
    }

    fn reverse_lookup(&self, ip: IpAddr) -> Option<Arc<NameRecord>> {
        self.by_a.get(&ip).cloned()
    }

    fn is_unique(&self, ip: IpAddr) -> bool {
        !self.by_a.contains_key(&ip)
    }

    pub fn register(
        &mut self,
        domain: String,
        ip: IpAddr,
    ) -> Result<Arc<NameRecord>, DNSRegistrationError> {
        if ip.is_loopback() {
            return Ok(Arc::new(NameRecord {
                a: ip,
                cname: domain,
            }));
        } else if is_restricted(ip) {
            return Err(DNSRegistrationError {
                domain,
                ip,
                reason: Reason::Restricted,
            });
        } else if !self.is_unique(ip) {
            return Err(DNSRegistrationError {
                domain,
                ip,
                reason: Reason::Duplicate,
            });
        }

        let record = Arc::new(NameRecord {
            a: ip,
            cname: domain,
        });
        self.insert(record.clone());
        Ok(record)
    }

    pub fn deregister(&mut self, record: Arc<NameRecord>) {
        if !record.a().is_loopback() {
            self.remove(record);
        }
    }

    fn insert(&mut self, record: Arc<NameRecord>) {
        self.by_a.insert(record.a(), record.clone());
        self.by_cname.insert(record.cname(), record.clone());
    }

    fn remove(&mut self, record: Arc<NameRecord>) {
        self.by_a.remove(&record.a());
        self.by_cname.remove(&record.cname());
    }
}

#[derive(Debug)]
enum Reason {
    Restricted,
    Duplicate,
}

#[derive(Debug)]
pub struct DNSRegistrationError {
    domain: String,
    ip: IpAddr,
    reason: Reason,
}

impl Display for DNSRegistrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self.reason {
            Reason::Restricted => "restricted",
            Reason::Duplicate => "duplicate",
        };
        write!(
            formatter,
            "invalid registration for {} ip `{}` to domain `{}` ",
            reason, self.ip, self.domain
        )
    }
}

pub struct Packet;
