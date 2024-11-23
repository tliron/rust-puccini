#![allow(unused)]

use {
    chrono::*,
    floria_plugin_sdk::data::*,
    std::{fmt, str::*},
};

//
// Timestamp
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA timestamp type represents a local instant in time containing two elements: the local
/// notation plus the time zone offset.
///
/// TOSCA timestamps are represented as strings following RFC 3339, which in turn uses a simplified
/// profile of ISO 8601. TOSCA adds an exception to RFC 3339: though RFC 3339 supports timestamps
/// with unknown local offsets, represented as the "-0" timezone, TOSCA does not support this
/// feature and will treat the unknown time zone as UTC. There are two reasons for this exception:
/// the first is that many systems do not support this distinction and TOSCA aims for
/// interoperability, and the second is that timestamps with unknown time zones cannot be converted
/// to UTC, making it impossible to apply comparison functions. If this feature is required, it can
/// be supported via a custom data type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// Datetime.
    pub datetime: DateTime<FixedOffset>,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.datetime.to_rfc3339(), formatter)
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        // Note: chrono treats "-00:00" as UTC, as expected by TOSCA
        let Ok(datetime) = DateTime::parse_from_rfc3339(representation) else {
            return Err("not RFC 3339".into());
        };

        Ok(Self { datetime })
    }
}

impl Into<Any> for Timestamp {
    fn into(self) -> Any {
        // Note: all the values here are either i32 or u32, so they will always be castable to i64
        normal_map!(
            ("year", self.datetime.year() as i64),
            ("month", self.datetime.month() as i64),
            ("day", self.datetime.day() as i64),
            ("hour", self.datetime.hour() as i64),
            ("minute", self.datetime.minute() as i64),
            ("second", self.datetime.second() as i64),
            ("nanosecond", self.datetime.nanosecond() as i64),
            ("utc-offset-seconds", self.datetime.offset().local_minus_utc() as i64),
        )
    }
}
