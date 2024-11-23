use {
    compris::resolve::*,
    kutil::{
        cli::depict::*,
        std::{immutable::*, string::*},
    },
    std::{fmt, io, str::*},
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
    pub qualifier: Option<ByteString>,

    /// Build. An optional integer value greater than or equal to 0 (zero) that can be used to
    /// further qualify different build versions of the code that has the same <qualifer_string>.
    pub build: Option<u64>,
}

impl Version {
    /// Constructor.
    pub fn new(major: u64, minor: u64, fix: Option<u64>, qualifier: Option<ByteString>, build: Option<u64>) -> Self {
        Self { major, minor, fix, qualifier, build }
    }
}

impl_resolve_from_str!(Version);

impl Depict for Version {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_number(writer, self.major)?;
        context.theme.write_delimiter(writer, '.')?;
        context.theme.write_number(writer, self.minor)?;

        if let Some(fix) = &self.fix {
            context.theme.write_delimiter(writer, '.')?;
            context.theme.write_number(writer, fix)?;
        }

        match &self.qualifier {
            Some(qualifier) => {
                context.theme.write_delimiter(writer, '-')?;
                context.theme.write_string(writer, qualifier)?;
                if let Some(build) = &self.build {
                    context.theme.write_number(writer, build)?;
                }
            }

            None => {
                if let Some(build) = &self.build {
                    context.theme.write_delimiter(writer, '-')?;
                    context.theme.write_number(writer, build)?;
                }
            }
        }

        Ok(())
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

impl FromStr for Version {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Our parsing method does no additional string allocation;
        // It's based on different slices in the same str

        let mut pre_dash = None;
        let mut post_dash = None;
        for (index, segment) in string.split('-').enumerate() {
            match index {
                0 => pre_dash = Some(segment),
                1 => post_dash = Some(segment),
                _ => {
                    return Err("has more than one \"-\"".into());
                }
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
                _ => {
                    return Err("has more than two \".\"".into());
                }
            }
        }

        if major.is_none() || minor.is_none() {
            return Err("does not have at least \"major.minor\"".into());
        }

        let mut version = Version::default();

        version.major = match major.expect("major").parse() {
            Ok(major) => major,
            Err(_) => {
                return Err("\"major\" is not a number".into());
            }
        };

        version.minor = match minor.expect("minor").parse() {
            Ok(minor) => minor,
            Err(_) => {
                return Err("\"minor\" is not a number".into());
            }
        };

        if let Some(fix) = fix {
            version.fix = match fix.parse() {
                Ok(fix) => Some(fix),
                Err(_) => {
                    return Err("\"fix\" is not a number".into());
                }
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
                        Err(_) => {
                            return Err("\"build\" is not a number".into());
                        }
                    };
                }

                None => {
                    version.qualifier = Some(post_dash.into());
                }
            }
        };

        Ok(version)
    }
}
