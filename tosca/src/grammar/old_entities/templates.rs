use super::{
    super::{errors::*, index::*, name::*, source::*},
    entity::*,
    id::*,
    template::*,
    r#type::*,
    types::*,
};

use {
    compris::annotate::*,
    kutil::std::{collections::*, error::*},
    std::{collections::*, marker::*, ptr},
};

//
// Templates
//

/// Container of template entities, e.g. node templates, groups, policies, property definitions, etc.,
/// anything with the [Template] trait.
///
/// We invented the word "entype" here to mean "applying a type" (to a template).
#[derive(Debug)]
pub struct Templates<'own, TemplateT, TypeT, CatalogT, AnnotatedT> {
    /// Maps [ID] to template.
    pub templates: FastHashMap<ID, &'own TemplateT>,

    /// Maps [ID] to type reference.
    pub types: FastHashMap<ID, OldEntityRef<'own, TypeT>>,

    /// Maps [ID] to complete template.
    pub complete: FastHashMap<ID, TemplateT>,

    catalog: PhantomData<CatalogT>,
    annotated: PhantomData<AnnotatedT>,
}

impl<'own, TemplateT, TypeT, CatalogT, AnnotatedT> Templates<'own, TemplateT, TypeT, CatalogT, AnnotatedT>
where
    TemplateT: Template<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    /// Add all templates.
    pub fn add_all(&mut self, source: &SourceID, templates: &'own BTreeMap<Name, TemplateT>) {
        for (name, template) in templates {
            self.add(ID::new(source.clone(), name.clone()), template);
        }
    }

    /// Add a template.
    pub fn add(&mut self, template_id: ID, template: &'own TemplateT) {
        self.templates.insert(template_id.into(), template);
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Whether we have a template.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn has(&self, template: &TemplateT) -> bool {
        for our_template in self.templates.values() {
            if ptr::eq(*our_template, template) {
                return true;
            }
        }
        false
    }

    /// Whether we have a template.
    pub fn has_id(&self, template_id: &ID) -> bool {
        self.templates.contains_key(template_id)
    }

    /// The template.
    pub fn get_by_id(&self, template_id: &ID) -> Option<&TemplateT> {
        self.templates.get(template_id).map(|template| *template)
    }

    /// The complete template.
    pub fn get_complete(&self, template_id: &ID) -> Option<&TemplateT> {
        self.complete.get(template_id)
    }

    /// The [ID] of the template.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_id(&self, template: &TemplateT) -> Option<&ID> {
        for (our_template_id, our_template) in &self.templates {
            if ptr::eq(*our_template, template) {
                return Some(our_template_id);
            }
        }
        None
    }

    /// The type of the template.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_type(&self, template: &TemplateT) -> Option<(&ID, &TypeT)> {
        self.get_id(template).and_then(|id| self.get_type_by_id(id))
    }

    /// The type of the template.
    pub fn get_type_by_id(&self, template_id: &ID) -> Option<(&ID, &TypeT)> {
        self.types.get(template_id).map(|template| (&template.id, template.entity))
    }

    /// The templates of the type.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_templates_by_type(&self, type_: &TypeT) -> Vec<&TemplateT> {
        let mut templates = Vec::default();
        for (our_template_id, type_ref) in &self.types {
            if ptr::eq(type_, type_ref.entity) {
                if let Some(template) = self.templates.get(our_template_id) {
                    templates.push(*template);
                }
            }
        }
        templates
    }

    /// The templates of the type.
    pub fn get_templates_by_type_id(&self, type_id: &ID) -> Vec<&TemplateT> {
        let mut templates = Vec::default();
        for (our_template_id, type_ref) in &self.types {
            if *type_id == type_ref.id {
                if let Some(template) = self.templates.get(our_template_id) {
                    templates.push(*template);
                }
            }
        }
        templates
    }

    /// The [ID]s of the type's templates.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_template_ids_by_type(&self, type_: &TypeT) -> Vec<&ID> {
        let mut template_ids = Vec::default();
        for (our_template_id, type_ref) in &self.types {
            if ptr::eq(type_, type_ref.entity) {
                template_ids.push(our_template_id)
            }
        }
        template_ids
    }

    /// The [ID]s of the type's templates.
    pub fn get_template_ids_by_type_id(&self, template_id: &ID) -> Vec<&ID> {
        let mut template_ids = Vec::default();
        for (our_template_id, type_ref) in &self.types {
            if *template_id == type_ref.id {
                template_ids.push(our_template_id)
            }
        }
        template_ids
    }

    /// Populate types.
    pub fn populate<'types, ErrorRecipientT>(
        &mut self,
        types: &Types<'types, TypeT, CatalogT, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
        'types: 'own,
    {
        for (template_id, template) in &self.templates {
            let type_name = template.get_type_name();
            match index.index.get(type_name) {
                Some(type_id) => match types.types.get(type_id) {
                    Some(type_) => {
                        self.types.insert(template_id.clone(), OldEntityRef::new(type_id.clone(), *type_));
                    }

                    None => errors.give(template.with_type_field_annotations(UnknownTypeError::new(
                        type_id.to_string(),
                        "templates.populate".into(),
                    )))?,
                },

                None => errors.give(template.with_type_field_annotations(UnknownTypeError::new(
                    type_name.to_string(),
                    "templates.populate".into(),
                )))?,
            }
        }

        Ok(())
    }

    /// Complete all templates.
    pub fn complete<ErrorRecipientT>(
        &self,
        types: &Types<TypeT, CatalogT, AnnotatedT>,
        catalog: &CatalogT,
        errors: &mut ErrorRecipientT,
    ) -> Result<FastHashMap<ID, TemplateT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_templates = FastHashMap::default();

        let template_ids: Vec<_> = self.templates.keys().cloned().collect();
        for template_id in template_ids {
            TemplateT::complete_into(&mut complete_templates, template_id, self, types, catalog, errors)?;
        }

        Ok(complete_templates)
    }
}

impl<'own, TemplateT, TypeT, CatalogT, AnnotatedT> Default for Templates<'own, TemplateT, TypeT, CatalogT, AnnotatedT> {
    fn default() -> Self {
        Self {
            templates: Default::default(),
            types: Default::default(),
            complete: Default::default(),
            catalog: PhantomData,
            annotated: PhantomData,
        }
    }
}
