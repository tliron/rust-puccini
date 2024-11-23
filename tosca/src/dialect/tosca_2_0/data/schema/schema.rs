use super::{
    super::{call::*, data_kind::*, expression::*},
    list::*,
    map::*,
    primitive::*,
    scalar::*,
    r#struct::*,
    value::*,
};

use kutil::cli::depict::*;

//
// Schema
//

/// Schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct Schema<AnnotatedT> {
    /// Values.
    #[depict(iter(item), as(depict))]
    pub values: Vec<ValueSchema<AnnotatedT>>,
}

impl<AnnotatedT> Schema<AnnotatedT> {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// [Some] if not empty.
    pub fn into_option(self) -> Option<Self> {
        if self.is_empty() { None } else { Some(self) }
    }

    /// Add a value schema and return its reference.
    pub fn add_value(&mut self, schema: ValueSchema<AnnotatedT>) -> SchemaReference {
        self.values.push(schema);
        self.values.len() - 1
    }

    /// Get the reference to an existing value schema if it exists. Otherwise, insert the schema
    /// and return its reference.
    pub fn get_or_add_value(&mut self, schema: ValueSchema<AnnotatedT>) -> SchemaReference
    where
        AnnotatedT: Clone,
    {
        for (index, existing) in self.values.iter().enumerate() {
            if schema == *existing {
                return index;
            }
        }

        self.add_value(schema)
    }

    /// Into validation.
    pub fn into_validation(self) -> Expression<AnnotatedT>
    where
        AnnotatedT: Default,
    {
        let call_value = Call::new_native_static("value");
        Call::new_native("_schema".into(), vec![call_value.into(), self.into()]).into()
    }
}

impl<AnnotatedT> From<Vec<ValueSchema<AnnotatedT>>> for Schema<AnnotatedT> {
    fn from(values: Vec<ValueSchema<AnnotatedT>>) -> Self {
        Self { values }
    }
}

impl<AnnotatedT> From<ValueSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(value: ValueSchema<AnnotatedT>) -> Self {
        Self::from(vec![value])
    }
}

impl<AnnotatedT> From<PrimitiveSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: PrimitiveSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<ScalarSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: ScalarSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<ListSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: ListSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<MapSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: MapSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<StructSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: StructSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<DataKind> for Schema<AnnotatedT> {
    fn from(data_kind: DataKind) -> Self {
        Self::from(ValueSchema::from(data_kind))
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for Schema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(mut self) -> Expression<AnnotatedT> {
        if self.values.len() == 1 {
            self.values.pop().expect("has one").into()
        } else {
            let list: Vec<_> = self.values.into_iter().map(|value| value.into()).collect();
            list.into()
        }
    }
}
