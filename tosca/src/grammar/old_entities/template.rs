use super::{
    super::{errors::*, name::*},
    id::*,
    templates::*,
    r#type::*,
    types::*,
};

use {
    compris::annotate::*,
    kutil::std::{collections::*, error::*},
};

///
#[derive(Debug)]
pub struct TemplateCompleteContext<'own, TypeT, CatalogT> {
    ///
    pub template_id: &'own ID,
    ///
    pub type_id: &'own ID,
    ///
    pub type_: &'own TypeT,
    ///
    pub catalog: &'own CatalogT,
}

//
// Template
//

/// A template is an entity that has a type.
pub trait Template<TypeT, CatalogT, AnnotatedT>
where
    Self: AnnotatedStruct + Clone + Sized,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    /// Type name.
    fn get_type_name(&self) -> &FullName;

    /// Complete.
    fn complete<ErrorRecipientT>(
        &self,
        context: TemplateCompleteContext<'_, TypeT, CatalogT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;

    /// Citation for the field containing the type name.
    fn get_type_name_field_annotations<'own>(&'own self) -> Option<&'own Annotations> {
        self.get_field_annotations("type_name")
    }

    /// Add the citation for the field containing the type name, if we have one.
    fn with_type_field_annotations<AnnotatedT2>(&self, annotated: AnnotatedT2) -> AnnotatedT2
    where
        AnnotatedT2: Annotated,
    {
        match self.get_type_name_field_annotations() {
            Some(annotations) => annotated.with_annotations(annotations.clone()),
            None => annotated,
        }
    }

    /// Complete.
    fn complete_into<ErrorRecipientT>(
        complete_templates: &mut FastHashMap<ID, Self>,
        template_id: ID,
        templates: &Templates<Self, TypeT, CatalogT, AnnotatedT>,
        types: &Types<TypeT, CatalogT, AnnotatedT>,
        catalog: &CatalogT,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        TypeT: Type<CatalogT, AnnotatedT>,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        if templates.complete.contains_key(&template_id) {
            return Ok(());
        }

        let type_id = match templates.get_type_by_id(&template_id) {
            Some((type_id, _type)) => Some(type_id),
            None => {
                tracing::warn!("template.complete: type not found for template: {}", template_id);
                None
            }
        };

        match templates.get_by_id(&template_id) {
            Some(template) => match type_id {
                Some(type_id) => match types.get_complete(&type_id) {
                    Some(derived_type) => {
                        let complete_template = template.complete(
                            TemplateCompleteContext {
                                template_id: &template_id,
                                type_id,
                                type_: derived_type,
                                catalog,
                            },
                            errors,
                        )?;
                        complete_templates.insert(template_id, complete_template);
                    }

                    None => tracing::warn!("template.complete: type not found: {}", type_id),
                },

                None => {
                    complete_templates.insert(template_id, template.clone());
                }
            },

            None => tracing::warn!("template.complete: template not found: {}", template_id),
        }

        Ok(())
    }
}
