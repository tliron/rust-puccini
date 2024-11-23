use super::super::{entity::*, name::*};

use {kutil::std::zerocopy::*, std::collections::*};

impl<EntityT, IntoEntityT> IntoScoped<BTreeMap<ByteString, IntoEntityT>> for BTreeMap<ByteString, EntityT>
where
    EntityT: IntoScoped<IntoEntityT>,
{
    fn into_scoped(&self, scope: &Scope) -> BTreeMap<ByteString, IntoEntityT> {
        self.iter().map(|(name, entity)| (name.clone(), entity.into_scoped(scope))).collect()
    }
}

impl IntoScoped<Option<Vec<FullName>>> for Option<Vec<FullName>> {
    fn into_scoped(&self, scope: &Scope) -> Option<Vec<FullName>> {
        self.as_ref().map(|vector| vector.iter().map(|full_name| full_name.clone().in_scope(scope.clone())).collect())
    }
}
