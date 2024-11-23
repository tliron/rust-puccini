use super::comparator::*;

use {
    chrono::*,
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, num::*, str::*},
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
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// Datetime.
    pub datetime: DateTime<FixedOffset>,
}

impl Timestamp {
    /// Constructor.
    pub fn new(datetime: DateTime<FixedOffset>) -> Self {
        Self { datetime }
    }
}

impl Comparator for Timestamp {
    fn comparator(&self) -> Expression {
        self.datetime.timestamp_micros().into()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.datetime.to_rfc3339(), formatter)
    }
}

// Conversions

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        // Note: chrono treats "-00:00" as UTC, as expected by TOSCA
        let Ok(datetime) = DateTime::parse_from_rfc3339(representation) else {
            return Err("not RFC 3339".into());
        };

        Ok(Self::new(datetime))
    }
}

impl TryFrom<Expression> for Timestamp {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Text(text) => text.parse(),
            Expression::Custom(custom_resource) => custom_resource.custom().try_into(),
            _ => Err(format!("timestamp not a string or custom: {}", expression.type_name())),
        }
    }
}

impl TryFrom<&Custom> for Timestamp {
    type Error = String;

    fn try_from(custom: &Custom) -> Result<Self, Self::Error> {
        if custom.kind != "timestamp" {
            return Err(format!("custom kind not \"timestamp\": {}", custom.kind));
        }

        let Expression::Map(map) = &custom.inner else {
            return Err("timestamp not a map".into());
        };
        let map = &map.map().inner;

        let year = get_integer(map, "year")?;
        let month = get_unsigned_integer(map, "month")?;
        let day = get_unsigned_integer(map, "day")?;
        let hour = get_unsigned_integer(map, "hour")?;
        let minute = get_unsigned_integer(map, "minute")?;
        let second = get_unsigned_integer(map, "second")?;
        let nanosecond = get_unsigned_integer(map, "nanosecond")?;
        let utc_offset_seconds = get_integer(map, "utc_offset_seconds")?;

        let Some(offset) = FixedOffset::east_opt(utc_offset_seconds) else {
            return Err(format!("timestamp has invalid \"utc_offset_seconds\" key: {}", utc_offset_seconds));
        };

        let MappedLocalTime::Single(datetime) = offset.with_ymd_and_hms(year, month, day, hour, minute, second) else {
            return Err("invalid timestamp".into());
        };

        let Some(datetime) = datetime.with_nanosecond(nanosecond) else {
            return Err(format!("timestamp has invalid \"nanosecond\" key: {}", nanosecond));
        };

        Ok(Self::new(datetime))
    }
}

impl Into<Expression> for Timestamp {
    fn into(self) -> Expression {
        // Note: all the values here are either i32 or u32, so they will always be castable to i64 and u64
        let map = BTreeMap::from([
            ("year".into(), self.datetime.year().into()),
            ("month".into(), self.datetime.month().into()),
            ("day".into(), self.datetime.day().into()),
            ("hour".into(), self.datetime.hour().into()),
            ("minute".into(), self.datetime.minute().into()),
            ("second".into(), self.datetime.second().into()),
            ("nanosecond".into(), self.datetime.nanosecond().into()),
            ("utc-offset-seconds".into(), self.datetime.offset().local_minus_utc().into()),
        ]);
        Custom::new("timestamp".into(), map.into()).into()
    }
}

fn get_integer(map: &BTreeMap<Expression, Expression>, name: &'static str) -> Result<i32, String> {
    match map.get(&name.into()) {
        Some(Expression::Integer(integer)) => {
            Ok((*integer).try_into().map_err(|error: TryFromIntError| error.to_string())?)
        }

        Some(Expression::UnsignedInteger(unsigned_integer)) => {
            Ok((*unsigned_integer).try_into().map_err(|error: TryFromIntError| error.to_string())?)
        }

        Some(expression) => {
            Err(format!("timestamp {:?} key not an integer or an unsigned integer: {}", name, expression.type_name()))
        }

        None => Err(format!("timestamp missing {:?} key", name)),
    }
}

fn get_unsigned_integer(map: &BTreeMap<Expression, Expression>, name: &'static str) -> Result<u32, String> {
    match map.get(&name.into()) {
        Some(Expression::Integer(integer)) => {
            Ok((*integer).try_into().map_err(|error: TryFromIntError| error.to_string())?)
        }

        Some(Expression::UnsignedInteger(unsigned_integer)) => {
            Ok((*unsigned_integer).try_into().map_err(|error: TryFromIntError| error.to_string())?)
        }

        Some(expression) => {
            Err(format!("timestamp {:?} key not an integer or an unsigned integer: {}", name, expression.type_name()))
        }

        None => Err(format!("timestamp missing {:?} key", name)),
    }
}
