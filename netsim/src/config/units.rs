use core::convert;
use std::fmt;
use std::time::Duration;

use serde::de::{self, Visitor};
use serde::Deserialize;

pub const NS: &'static str = "ns";
pub const US: &'static str = "us";
pub const MS: &'static str = "ms";
pub const S: &'static str = "s";
pub const MIN: &'static str = "min";
pub const H: &'static str = "h";

pub const KBIT: &'static str = "kbit";
pub const MBIT: &'static str = "mbit";
pub const GBIT: &'static str = "gbit";
pub const TBIT: &'static str = "tbit";

pub const KIBIT: &'static str = "kibit";
pub const MIBIT: &'static str = "mibit";
pub const GIBIT: &'static str = "gibit";
pub const TIBIT: &'static str = "tibit";

pub const KBYTE: &'static str = "kbyte";
pub const MBYTE: &'static str = "mbyte";
pub const GBYTE: &'static str = "gbyte";
pub const TBYTE: &'static str = "tbyte";

pub const KIBYTE: &'static str = "kibyte";
pub const MIBYTE: &'static str = "mibyte";
pub const GIBYTE: &'static str = "gibyte";
pub const TIBYTE: &'static str = "tibyte";

#[derive(Debug)]
pub struct TimeInterval(Duration);

impl<'de> Deserialize<'de> for TimeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeIntervalVisitor)
    }
}

impl convert::From<TimeInterval> for Duration {
    fn from(interval: TimeInterval) -> Self {
        interval.0
    }
}

impl convert::From<Duration> for TimeInterval {
    fn from(interval: Duration) -> Self {
        Self(interval)
    }
}

#[derive(Debug)]
struct InvalidTimeInterval {
    span: String,
}

impl std::error::Error for InvalidTimeInterval {}

impl fmt::Display for InvalidTimeInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid time interval (\"{}\")", self.span)
    }
}

impl de::Error for InvalidTimeInterval {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            span: msg.to_string(),
        }
    }
}

struct TimeIntervalVisitor;

impl<'de> Visitor<'de> for TimeIntervalVisitor {
    type Value = TimeInterval;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`uint` (ns | us | ms | s | min | h)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_borrowed_str(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = if value.ends_with(NS) {
            value[..value.len() - NS.len()]
                .trim_end()
                .parse::<u64>()
                .map(Duration::from_nanos)
        } else if value.ends_with(US) {
            value[..value.len() - US.len()]
                .trim_end()
                .parse::<u64>()
                .map(Duration::from_micros)
        } else if value.ends_with(MS) {
            value[..value.len() - MS.len()]
                .trim_end()
                .parse::<u64>()
                .map(Duration::from_millis)
        } else if value.ends_with(S) {
            value[..value.len() - S.len()]
                .trim_end()
                .parse::<u64>()
                .map(Duration::from_secs)
        } else if value.ends_with(MIN) {
            value[..value.len() - MIN.len()]
                .trim_end()
                .parse::<u64>()
                .map(|mins| Duration::from_secs(mins * 60))
        } else if value.ends_with(H) {
            value[..value.len() - H.len()]
                .trim_end()
                .parse::<u64>()
                .map(|hours| Duration::from_secs(hours * 60 * 60))
        } else {
            return Err(E::custom(value.to_string()));
        };

        result
            .map(TimeInterval::from)
            .map_err(|err| E::custom(err.to_string()))
    }
}

#[derive(Debug)]
pub struct Bandwidth(u64);

impl Bandwidth {
    const BITS_PER_KBIT: u64 = 10u64.pow(1);
    const BITS_PER_MBIT: u64 = 10u64.pow(2);
    const BITS_PER_GBIT: u64 = 10u64.pow(3);
    const BITS_PER_TBIT: u64 = 10u64.pow(4);

    const BITS_PER_KIBIT: u64 = 2u64.pow(10);
    const BITS_PER_MIBIT: u64 = 2u64.pow(20);
    const BITS_PER_GIBIT: u64 = 2u64.pow(30);
    const BITS_PER_TIBIT: u64 = 2u64.pow(40);

    const fn bit_per_second(&self) -> u64 {
        self.0
    }

    const fn from_kbit(kbit: u64) -> Self {
        Self(kbit.saturating_mul(Self::BITS_PER_KBIT))
    }

    const fn from_mbit(mbit: u64) -> Self {
        Self(mbit.saturating_mul(Self::BITS_PER_MBIT))
    }

    const fn from_gbit(gbit: u64) -> Self {
        Self(gbit.saturating_mul(Self::BITS_PER_GBIT))
    }

    const fn from_tbit(tbit: u64) -> Self {
        Self(tbit.saturating_mul(Self::BITS_PER_TBIT))
    }

    const fn from_kibit(kibit: u64) -> Self {
        Self(kibit.saturating_mul(Self::BITS_PER_KIBIT))
    }

    const fn from_mibit(mibit: u64) -> Self {
        Self(mibit.saturating_mul(Self::BITS_PER_MIBIT))
    }

    const fn from_gibit(gibit: u64) -> Self {
        Self(gibit.saturating_mul(Self::BITS_PER_GIBIT))
    }

    const fn from_tibit(tibit: u64) -> Self {
        Self(tibit.saturating_mul(Self::BITS_PER_TIBIT))
    }
}

impl<'de> Deserialize<'de> for Bandwidth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BandwidthVisitor)
    }
}

#[derive(Debug)]
struct InvalidBandwidth {
    span: String,
}

impl std::error::Error for InvalidBandwidth {}

impl fmt::Display for InvalidBandwidth {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid bandwidth (\"{}\")", self.span)
    }
}

impl de::Error for InvalidBandwidth {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            span: msg.to_string(),
        }
    }
}

struct BandwidthVisitor;

impl<'de> Visitor<'de> for BandwidthVisitor {
    type Value = Bandwidth;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`uint` (kbit | mbit | gbit | tbit | kibit | mibit | gibit | tibit)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_borrowed_str(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = if value.ends_with(KBIT) {
            value[..value.len() - KBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_kbit)
        } else if value.ends_with(MBIT) {
            value[..value.len() - MBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_mbit)
        } else if value.ends_with(GBIT) {
            value[..value.len() - GBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_gbit)
        } else if value.ends_with(TBIT) {
            value[..value.len() - TBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_tbit)
        } else if value.ends_with(KIBIT) {
            value[..value.len() - KIBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_kibit)
        } else if value.ends_with(MIBIT) {
            value[..value.len() - MIBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_mibit)
        } else if value.ends_with(GIBIT) {
            value[..value.len() - GIBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_gibit)
        } else if value.ends_with(TIBIT) {
            value[..value.len() - TIBIT.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bandwidth::from_tibit)
        } else {
            return Err(E::custom(value.to_string()));
        };

        result.map_err(|err| E::custom(err.to_string()))
    }
}

#[derive(Debug)]
pub struct Bytes(u64);

impl Bytes {
    const BYTES_PER_KBYTE: u64 = 10u64.pow(1);
    const BYTES_PER_MBYTE: u64 = 10u64.pow(2);
    const BYTES_PER_GBYTE: u64 = 10u64.pow(3);
    const BYTES_PER_TBYTE: u64 = 10u64.pow(4);

    const BYTES_PER_KIBYTE: u64 = 2u64.pow(10);
    const BYTES_PER_MIBYTE: u64 = 2u64.pow(20);
    const BYTES_PER_GIBYTE: u64 = 2u64.pow(30);
    const BYTES_PER_TIBYTE: u64 = 2u64.pow(40);

    const fn bytes(&self) -> u64 {
        self.0
    }

    const fn from_kbyte(kbyte: u64) -> Self {
        Self(kbyte.saturating_mul(Self::BYTES_PER_KBYTE))
    }

    const fn from_mbyte(mbyte: u64) -> Self {
        Self(mbyte.saturating_mul(Self::BYTES_PER_MBYTE))
    }

    const fn from_gbyte(gbyte: u64) -> Self {
        Self(gbyte.saturating_mul(Self::BYTES_PER_GBYTE))
    }

    const fn from_tbyte(tbyte: u64) -> Self {
        Self(tbyte.saturating_mul(Self::BYTES_PER_TBYTE))
    }

    const fn from_kibyte(kibyte: u64) -> Self {
        Self(kibyte.saturating_mul(Self::BYTES_PER_KIBYTE))
    }

    const fn from_mibyte(mbyte: u64) -> Self {
        Self(mbyte.saturating_mul(Self::BYTES_PER_MIBYTE))
    }

    const fn from_gibyte(gibyte: u64) -> Self {
        Self(gibyte.saturating_mul(Self::BYTES_PER_GIBYTE))
    }

    const fn from_tibyte(tibyte: u64) -> Self {
        Self(tibyte.saturating_mul(Self::BYTES_PER_TIBYTE))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BytesVisitor)
    }
}

#[derive(Debug)]
struct InvalidBytesUnit {
    span: String,
}

impl std::error::Error for InvalidBytesUnit {}

impl fmt::Display for InvalidBytesUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid bytes unit (\"{}\")", self.span)
    }
}

impl de::Error for InvalidBytesUnit {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            span: msg.to_string(),
        }
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("`uint` (kbyte | mbyte | gbyte | tbyte | kibyte | mibyte | gibyte | tibyte)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_borrowed_str(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = if value.ends_with(KBYTE) {
            value[..value.len() - KBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_kbyte)
        } else if value.ends_with(MBYTE) {
            value[..value.len() - MBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_mbyte)
        } else if value.ends_with(GBYTE) {
            value[..value.len() - GBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_gbyte)
        } else if value.ends_with(TBYTE) {
            value[..value.len() - TBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_tbyte)
        } else if value.ends_with(KIBYTE) {
            value[..value.len() - KIBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_kibyte)
        } else if value.ends_with(MIBYTE) {
            value[..value.len() - MIBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_mibyte)
        } else if value.ends_with(GIBYTE) {
            value[..value.len() - GIBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_gibyte)
        } else if value.ends_with(TIBYTE) {
            value[..value.len() - TIBYTE.len()]
                .trim_end()
                .parse::<u64>()
                .map(Bytes::from_tibyte)
        } else {
            return Err(E::custom(value.to_string()));
        };

        result.map_err(|err| E::custom(err.to_string()))
    }
}
