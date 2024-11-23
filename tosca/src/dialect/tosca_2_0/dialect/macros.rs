/// Compile type.
#[macro_export]
macro_rules! compile_type_2_0 (
    (
        $kind:ident,
        $entity:expr,
        $tosca_type:ident,
        $entity_kind:ident,
        $full_name:ident,
        $floria_store:ident,
        $floria_type:ident,
        $depot:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        if $entity_kind == $kind {
            use $crate::grammar::FloriaToscaMetadata;

            tracing::debug!(
                source = $source_id.to_string(),
                name = $full_name.to_string(),
                type = $entity,
                "compiling"
            );

            $floria_type.metadata.set_tosca_entity($entity);

            match $depot.get_entity::<$tosca_type<AnnotatedT>, _>($kind, &$full_name, &$source_id) {
                Ok(tosca_type) => {
                    $floria_type.metadata.set_tosca_description(tosca_type.description.as_ref());
                    $floria_type.metadata.merge_tosca_metadata(&tosca_type.metadata);

                    if let Some(derived_from) = &tosca_type.derived_from {
                        $floria_type.metadata.set_tosca_parent(&derived_from.to_string());
                    }
                }

                Err(error) => $errors.give(error)?,
            }

            unwrap_or_give_and_return!($floria_store.add_group($floria_type), $errors, Ok(None));

            continue;
        }
    }
);
