use super::{
    super::{super::super::grammar::*, data::*, entities::*},
    entity_kind::*,
};

use {
    compris::{annotate::*, normal::*},
    std::fmt,
};

impl super::Dialect {
    /// Create the implicit source.
    pub fn implicit_source<AnnotatedT>() -> Source
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut source = Source::new(SourceID::Internal(super::DIALECT_ID), super::DIALECT_ID);

        for name in
            ["string", "integer", "float", "boolean", "bytes", "nil", "timestamp", "scalar", "version", "list", "map"]
        {
            let mut data_type = DataType::<AnnotatedT>::default();

            let value = Call::new(get_dispatch_name("value").into(), Default::default());
            let validation =
                Call::new(get_dispatch_name("_is_a").into(), vec![value.into(), Variant::from(name).into()]);
            data_type.validation = Some(validation.into());

            data_type.kind = Some(match name {
                "timestamp" => DataTypeKind::Timestamp,
                "scalar" => DataTypeKind::Scalar,
                "version" => DataTypeKind::Version,
                "list" => DataTypeKind::List,
                "map" => DataTypeKind::Map,
                _ => DataTypeKind::Primitive,
            });

            source.add_entity::<_, AnnotatedT>(DATA_TYPE, name.into(), data_type).expect("add_entity");
        }

        source
    }
}
