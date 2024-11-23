use super::{
    super::{data_kind::*, expression::*},
    list::*,
    map::*,
    primitive::*,
    scalar::*,
    r#struct::*,
};

use kutil::cli::depict::*;

/// Schema reference.
pub type SchemaReference = usize;

//
// ValueSchema
//

/// Value schema.
#[derive(Clone, Debug, Depict)]
pub enum ValueSchema<AnnotatedT> {
    /// Reference.
    #[depict(style(number))]
    Reference(SchemaReference),

    /// Primitive.
    #[depict(as(depict))]
    Primitive(PrimitiveSchema<AnnotatedT>),

    /// Scalar.
    #[depict(as(depict))]
    Scalar(ScalarSchema<AnnotatedT>),

    /// List.
    #[depict(as(depict))]
    List(ListSchema<AnnotatedT>),

    /// Map.
    #[depict(as(depict))]
    Map(MapSchema<AnnotatedT>),

    /// Struct.
    #[depict(as(depict))]
    Struct(StructSchema<AnnotatedT>),
}

impl<AnnotatedT> PartialEq for ValueSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(primitive), Self::Primitive(other_primitive)) => primitive == other_primitive,
            (Self::Scalar(scalar), Self::Scalar(other_scalar)) => scalar == other_scalar,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            (Self::Struct(struct_), Self::Struct(other_struct)) => struct_ == other_struct,
            _ => false,
        }
    }
}

impl<AnnotatedT> From<PrimitiveSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: PrimitiveSchema<AnnotatedT>) -> Self {
        Self::Primitive(schema)
    }
}

impl<AnnotatedT> From<ScalarSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: ScalarSchema<AnnotatedT>) -> Self {
        Self::Scalar(schema)
    }
}

impl<AnnotatedT> From<ListSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: ListSchema<AnnotatedT>) -> Self {
        Self::List(schema)
    }
}

impl<AnnotatedT> From<MapSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: MapSchema<AnnotatedT>) -> Self {
        Self::Map(schema)
    }
}

impl<AnnotatedT> From<StructSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: StructSchema<AnnotatedT>) -> Self {
        Self::Struct(schema)
    }
}

impl<AnnotatedT> From<DataKind> for ValueSchema<AnnotatedT> {
    fn from(data_kind: DataKind) -> Self {
        Self::Primitive(data_kind.into())
    }
}

impl<AnnotatedT> From<SchemaReference> for ValueSchema<AnnotatedT> {
    fn from(reference: SchemaReference) -> Self {
        Self::Reference(reference)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for ValueSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        match self {
            // The only schema that can be represented as an unsigned integer
            Self::Reference(reference) => (reference as u64).into(),

            // The only schema that can be represented as text
            Self::Primitive(primitive) => primitive.into(),

            Self::Scalar(scalar) => scalar.into(),
            Self::List(list) => list.into(),
            Self::Map(map) => map.into(),
            Self::Struct(struct_) => struct_.into(),
        }
    }
}
