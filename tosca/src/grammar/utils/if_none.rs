use compris::annotate::*;

/// Clone [StructAnnotations].
pub fn clone_struct_annotations(struct_annotations: &StructAnnotations, field_names: &[&str]) -> StructAnnotations {
    let mut clone = StructAnnotations::default();
    for field_name in field_names {
        if let Some(annotations) = struct_annotations.get(*field_name) {
            clone.insert((*field_name).into(), annotations.clone());
        }
    }
    clone
}

/// Clone if [None].
#[macro_export]
macro_rules! if_none_clone {
    (
        $field:tt,
        $self:ident,
        $entity:ident $(,)?
    ) => {{
        if $self.$field.is_none() && !$entity.$field.is_some() {
            $self.$field = $entity.$field.clone();
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self.annotations.insert(stringify!($field).into(), annotations.clone());
            }
        }
    }};
}

/// Clone if `is_empty`.
#[macro_export]
macro_rules! if_empty_clone {
    (
        $field:tt,
        $self:ident,
        $entity:ident $(,)?
    ) => {
        if $self.$field.is_empty() && !$entity.$field.is_empty() {
            $self.$field = $entity.$field.clone();
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self.annotations.insert(stringify!($field).into(), annotations.clone());
            }
        }
    };
}

/// Do something if [None].
#[macro_export]
macro_rules! if_none_else {
    (
        $field:tt,
        $self:ident,
        $entity:ident,
        $( $code:tt )* $(,)?
    ) => {
        if $self.$field.is_none() {
            $self.$field = $( $code )*;
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self.annotations.insert(stringify!($field).into(), annotations.clone());
            }
        }
    };
}

#[allow(unused_imports)]
pub use {if_empty_clone, if_none_clone, if_none_else};
