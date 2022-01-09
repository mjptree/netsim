use std::time;

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SimulationTime(Duration);

impl SimulationTime {
    pub fn from_nanos(nanos: i64) -> Self {
        Self(Duration::nanoseconds(nanos))
    }

    pub fn from_micros(micros: i64) -> Self {
        Self(Duration::microseconds(micros))
    }

    pub fn from_millis(millis: i64) -> Self {
        Self(Duration::milliseconds(millis))
    }
}

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
        let mut lhs = self.0;
        let rhs = other.0;

        while lhs > rhs {
            lhs = lhs - rhs;
        }

        rhs.into()
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct EmulatedTime(DateTime<Utc>);

lazy_static! {
    pub static ref UNIX_EPOCH: EmulatedTime =
        NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0).into();
    pub static ref SIMULATION_START: EmulatedTime =
        NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0).into();
}

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
        if let Self::Running { start, lapsed } = *self {
            *self = Self::Paused(time::Instant::now() - start + lapsed);
        }
    }

    pub fn resume(&mut self) {
        if let Self::Paused(lapsed) = *self {
            *self = Self::Running {
                start: time::Instant::now(),
                lapsed,
            }
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

    pub fn reset(&mut self, start_paused: bool) {
        *self = if start_paused {
            Self::new()
        } else {
            Self::start()
        }
    }
}
