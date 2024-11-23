use super::comparator::*;

use {
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, num::*, str::*},
};

//
// Version
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA version type represents a version string. TOSCA versions provide a normative means to
/// represent a version string which enables the comparison and management of version information
/// over time.
#[derive(Clone, Debug, Default)]
pub struct Version {
    /// Major. A mandatory integer value greater than or equal to 0 (zero).
    pub major: u64,

    /// Minor. A mandatory integer value greater than or equal to 0 (zero).
    pub minor: u64,

    /// Fix. An optional integer value greater than or equal to 0 (zero).
    pub fix: Option<u64>,

    /// Qualifier. an optional string that indicates a named, pre-release version of the associated
    /// code that has been derived from the version of the code identified by the combination
    /// <major_version>, <minor_version> and <fix_version> numbers.
    pub qualifier: Option<String>,

    /// Build. An optional integer value greater than or equal to 0 (zero) that can be used to
    /// further qualify different build versions of the code that has the same <qualifer_string>.
    pub build: Option<u64>,
}

impl Version {
    /// Constructor.
    #[allow(unused)]
    pub fn new(major: u64, minor: u64, fix: Option<u64>, qualifier: Option<String>, build: Option<u64>) -> Self {
        Self { major, minor, fix, qualifier, build }
    }
}

impl Comparator for Version {
    fn comparator(&self) -> Expression {
        normal_vec!(
            self.major,
            self.minor,
            self.fix.map(|fix| fix.into()).unwrap_or(Expression::Null),
            self.qualifier.as_ref().map(|qualifier| qualifier.clone().into()).unwrap_or(Expression::Null),
            self.build.map(|build| build.into()).unwrap_or(Expression::Null),
        )
        .into()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)?;

        if let Some(fix) = &self.fix {
            write!(formatter, ".{}", fix)?;
        }

        match &self.qualifier {
            Some(qualifier) => match &self.build {
                Some(build) => write!(formatter, "-{}{}", qualifier, build)?,
                None => write!(formatter, "-{}", qualifier)?,
            },

            None => {
                if let Some(build) = &self.build {
                    write!(formatter, "-{}", build)?;
                }
            }
        }

        Ok(())
    }
}

// Conversions

impl FromStr for Version {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Our parsing method does no additional string allocation;
        // It's based on different slices in the same str

        let mut pre_dash = None;
        let mut post_dash = None;
        for (index, segment) in string.split('-').enumerate() {
            match index {
                0 => pre_dash = Some(segment),
                1 => post_dash = Some(segment),
                _ => return Err("version has more than one \"-\"".into()),
            }
        }

        let mut major = None;
        let mut minor = None;
        let mut fix = None;
        for (index, segment) in pre_dash.expect("pre_dash").split('.').enumerate() {
            match index {
                0 => major = Some(segment),
                1 => minor = Some(segment),
                2 => fix = Some(segment),
                _ => return Err("version has more than two \".\"".into()),
            }
        }

        if major.is_none() || minor.is_none() {
            return Err("version does not have at least \"major.minor\"".into());
        }

        let mut version = Version::default();

        version.major = match major.expect("major").parse() {
            Ok(major) => major,
            Err(_) => return Err("version \"major\" key not a number".into()),
        };

        version.minor = match minor.expect("minor").parse() {
            Ok(minor) => minor,
            Err(_) => return Err("version \"minor\" key not a number".into()),
        };

        if let Some(fix) = fix {
            version.fix = match fix.parse() {
                Ok(fix) => Some(fix),
                Err(_) => return Err("version \"fix\" key not a number".into()),
            };
        };

        if let Some(post_dash) = post_dash {
            // The spec is unclear, but we will assume the qualifier cannot have digits
            // In other words, the build number starts with the first digit
            match post_dash.find(|c: char| c.is_digit(10)) {
                Some(number_start) => {
                    version.qualifier = Some(post_dash[..number_start].into());

                    version.build = match post_dash[number_start..].parse() {
                        Ok(build) => Some(build),
                        Err(_) => return Err("version \"build\" key not a number".into()),
                    };
                }

                None => version.qualifier = Some(post_dash.into()),
            }
        };

        Ok(version)
    }
}

impl TryFrom<Expression> for Version {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Text(text) => text.parse(),
            Expression::Custom(custom_resource) => custom_resource.custom().try_into(),
            _ => Err(format!("version not a string or custom: {}", expression.type_name())),
        }
    }
}

impl TryFrom<&Custom> for Version {
    type Error = String;

    fn try_from(custom: &Custom) -> Result<Self, Self::Error> {
        if custom.kind != "version" {
            return Err(format!("custom kind not \"version\": {}", custom.kind));
        }

        let Expression::Map(map) = &custom.inner else {
            return Err("version not a map".into());
        };
        let map = &map.map().inner;

        let major = get_unsigned_integer(map, "major")?;
        let minor = get_unsigned_integer(map, "minor")?;
        let fix = get_unsigned_integer_option(map, "fix")?;
        let qualifier = get_string_option(map, "qualifier")?;
        let build = get_unsigned_integer_option(map, "build")?;

        Ok(Self::new(major, minor, fix, qualifier, build))
    }
}

impl Into<Expression> for Version {
    fn into(self) -> Expression {
        let mut map = BTreeMap::from([("major".into(), self.major.into()), ("minor".into(), self.minor.into())]);

        if let Some(fix) = self.fix {
            map.insert("fix".into(), fix.into());
        }

        if let Some(qualifier) = self.qualifier {
            map.insert("qualifier".into(), qualifier.into());
        }

        if let Some(build) = self.build {
            map.insert("build".into(), build.into());
        }

        Custom::new("version".into(), map.into()).into()
    }
}

fn get_unsigned_integer(map: &BTreeMap<Expression, Expression>, name: &'static str) -> Result<u64, String> {
    match get_unsigned_integer_option(map, name)? {
        Some(unsigned_integer) => Ok(unsigned_integer),
        None => Err(format!("version missing {:?} key", name)),
    }
}

fn get_unsigned_integer_option(
    map: &BTreeMap<Expression, Expression>,
    name: &'static str,
) -> Result<Option<u64>, String> {
    match map.get(&name.into()) {
        Some(Expression::Integer(integer)) => {
            Ok(Some((*integer).try_into().map_err(|error: TryFromIntError| error.to_string())?))
        }

        Some(Expression::UnsignedInteger(unsigned_integer)) => Ok(Some(*unsigned_integer)),

        Some(expression) => {
            Err(format!("version {:?} key not an integer or an unsigned integer: {}", name, expression.type_name()))
        }

        None => Ok(None),
    }
}

fn get_string_option(map: &BTreeMap<Expression, Expression>, name: &'static str) -> Result<Option<String>, String> {
    match map.get(&name.into()) {
        Some(Expression::Text(text)) => Ok(Some(text.clone())),
        Some(expression) => Err(format!("version {:?} key not a string: {}", name, expression.type_name())),
        None => Ok(None),
    }
}
