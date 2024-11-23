use super::dialect::*;

use {
    kutil::std::any::*,
    std::{any::*, fmt},
};

//
// DialectRef
//

/// Common reference type for [Dialect].
pub type DialectRef = Box<dyn Dialect>;

impl fmt::Debug for DialectRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "DialectRef")
    }
}

impl<GrammarT> From<GrammarT> for DialectRef
where
    GrammarT: 'static + Dialect,
{
    fn from(value: GrammarT) -> Self {
        Box::new(value)
    }
}

impl DowncastRef for DialectRef {
    fn downcast_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: Any,
    {
        (self.as_ref() as &dyn Any).downcast_ref()
    }
}
