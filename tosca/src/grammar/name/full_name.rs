use super::{name::*, scope::*};

use {
    compris::impl_resolve_from_str,
    kutil::{cli::depict::*, std::string::*},
    std::{fmt, io, str::*},
};

//
// FullName
//

/// [Name] with [Scope].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullName {
    /// Scope.
    pub scope: Scope,

    /// Name.
    pub name: Name,
}

impl FullName {
    /// Constructor.
    pub fn new(scope: Scope, name: Name) -> Self {
        Self { scope, name }
    }

    /// True if name is empty.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Add a prefix to the [Scope].
    pub fn in_scope(self, mut prefix: Scope) -> Self {
        prefix.segments.extend(self.scope.segments);
        Self::new(prefix, self.name)
    }

    /// To Floria class ID.
    pub fn to_floria_class_id(&self, prefix: &str) -> floria::ID {
        let (mut directory, name) = (self.scope.to_floria_directory(), self.name.clone());
        directory.add_first_segment(prefix.into());
        directory.add_first_segment("tosca".into());
        floria::ID::new_for(floria::Kind::Class, directory, name.into())
    }
}

impl Depict for FullName {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        self.scope.depict(writer, context)?;
        self.name.depict(writer, &context.clone().with_separator(false))
    }
}

impl fmt::Display for FullName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.scope, self.name)
    }
}

impl FromStr for FullName {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = string.split(":").collect();
        let length = segments.len();

        Ok(if length > 0 {
            let mut scope = Vec::with_capacity(length - 1);
            for segment in &segments[..length - 1] {
                scope.push(segment.parse()?);
            }

            Self::new(Scope::from(scope), segments[length - 1].parse()?)
        } else {
            Self::new(Default::default(), string.parse()?)
        })
    }
}

impl From<Name> for FullName {
    fn from(name: Name) -> Self {
        Self::new(Default::default(), name)
    }
}

impl_resolve_from_str!(FullName);
