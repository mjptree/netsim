use std::time;

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

pub struct SimulationTime(Duration);

impl From<Duration> for SimulationTime {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl std::ops::Add<Self> for SimulationTime {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0).into()
    }
}

impl std::ops::Sub<Self> for SimulationTime {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0).into()
    }
}

impl std::ops::Rem<Self> for SimulationTime {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = other.0;

        while lhs > rhs {
            lhs = lhs - rhs;
        }

        rhs.into()
    }
}

#[derive(PartialEq, PartialOrd)]
pub struct EmulatedTime(DateTime<Utc>);

pub const UNIX_EPOCH: EmulatedTime = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0).into();
pub const SIMULATION_START: EmulatedTime = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0).into();

impl EmulatedTime {
    pub fn duration_since(&self, time: &EmulatedTime) -> Duration {
        self.0.signed_duration_since(time.0)
    }
}

impl From<DateTime<Utc>> for EmulatedTime {
    fn from(instant: DateTime<Utc>) -> Self {
        Self(instant)
    }
}

impl From<NaiveDateTime> for EmulatedTime {
    fn from(instant: NaiveDateTime) -> Self {
        DateTime::from_utc(instant, Utc).into()
    }
}

impl From<SimulationTime> for EmulatedTime {
    fn from(time: SimulationTime) -> Self {
        Self(SIMULATION_START.0 + time.0)
    }
}

impl From<EmulatedTime> for SimulationTime {
    fn from(time: EmulatedTime) -> Self {
        Self(time.duration_since(&SIMULATION_START))
    }
}

pub enum PerfTimer {
    Running {
        start: time::Instant,
        lapsed: time::Duration,
    },
    Paused(time::Duration),
}

impl PerfTimer {
    pub fn new() -> Self {
        Self::Paused(time::Duration::from_nanos(0))
    }

    pub fn start() -> Self {
        Self::Running {
            start: time::Instant::now(),
            lapsed: time::Duration::from_nanos(0),
        }
    }

    pub fn pause(&mut self) {
        *self = match *self {
            Self::Running { start, lapsed } => Self::Paused(time::Instant::now() - start + lapsed),
            _ => *self,
        }
    }

    pub fn resume(&mut self) {
        *self = match *self {
            Self::Paused(lapsed) => Self::Running {
                start: time::Instant::now(),
                lapsed,
            },
            _ => *self,
        }
    }

    pub fn lapsed(&self) -> time::Duration {
        match *self {
            Self::Running { start, lapsed } => time::Instant::now() - start + lapsed,
            Self::Paused(lapsed) => lapsed,
        }
    }

    pub fn stop(self) -> time::Duration {
        match self {
            Self::Running { start, lapsed } => time::Instant::now() - start + lapsed,
            Self::Paused(lapsed) => lapsed,
        }
    }
}
