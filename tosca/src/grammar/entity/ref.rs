use super::entity::*;

use {
    kutil::std::any::*,
    std::{any::*, fmt},
};

//
// EntityRef
//

/// Common reference type for [Entity].
pub type EntityRef = Box<dyn Entity>;

impl fmt::Debug for EntityRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "EntityRef")
    }
}

impl<EntityT> From<EntityT> for EntityRef
where
    EntityT: 'static + Entity,
{
    fn from(value: EntityT) -> Self {
        Box::new(value)
    }
}

impl DowncastRef for EntityRef {
    fn downcast_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: Any,
    {
        (self.as_ref() as &dyn Any).downcast_ref()
    }
}
