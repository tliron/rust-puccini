use super::super::{entity::*, name::*};

use {kutil::std::immutable::*, std::collections::*};

impl<EntityT, IntoEntityT> ConvertIntoScope<BTreeMap<ByteString, IntoEntityT>> for BTreeMap<ByteString, EntityT>
where
    EntityT: ConvertIntoScope<IntoEntityT>,
{
    fn convert_into_scope(&self, scope: &Scope) -> BTreeMap<ByteString, IntoEntityT> {
        self.iter().map(|(name, entity)| (name.clone(), entity.convert_into_scope(scope))).collect()
    }
}

impl ConvertIntoScope<Option<Vec<FullName>>> for Option<Vec<FullName>> {
    fn convert_into_scope(&self, scope: &Scope) -> Option<Vec<FullName>> {
        self.as_ref().map(|vector| vector.iter().map(|full_name| full_name.clone().in_scope(scope.clone())).collect())
    }
}
