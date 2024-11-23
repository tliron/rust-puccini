use super::{
    super::{errors::*, index::*, name::*, source::*},
    id::*,
    template::*,
    templates::*,
    r#type::*,
};

use {
    compris::annotate::*,
    kutil::{
        cli::debug::*,
        std::{collections::*, error::*, iter::*, zerocopy::*},
    },
    std::{collections::*, io, marker::*, ptr},
};

//
// Types
//

/// Container of type entities, e.g. node type, relationship type, capability type, etc., anything
/// with the [Type] trait.
///
/// Maintains a hierarchy, a tree structure for which entities can have a single "parent". Entities
/// without parents are the "roots" of the tree.
///
/// The hierarchy *cannot* be cyclical, i.e. a node can't be its own ancestor/descendent.
#[derive(Debug)]
pub struct Types<'own, TypeT, CatalogT, AnnotatedT> {
    /// Maps [ID] to type.
    pub types: FastHashMap<ID, &'own TypeT>,

    /// Maps [ID] to parent type.
    pub parents: FastHashMap<ID, &'own TypeT>,

    /// Maps [ID] to complete type.
    pub complete: FastHashMap<ID, TypeT>,

    catalog: PhantomData<CatalogT>,
    annotated: PhantomData<AnnotatedT>,
}

impl<'own, TypeT, CatalogT, AnnotatedT> Types<'own, TypeT, CatalogT, AnnotatedT>
where
    TypeT: Type<CatalogT, AnnotatedT>,
{
    /// Add all types.
    pub fn add_all(&mut self, source: &SourceID, types: &'own BTreeMap<Name, TypeT>) {
        for (type_id, type_) in types {
            self.add(ID::new(source.clone(), type_id.clone()), type_);
        }
    }

    /// Add a type.
    pub fn add(&mut self, id: ID, type_: &'own TypeT) {
        // TODO: "ambiguity" error if name already exists
        tracing::info!("add: {}", id);
        self.types.insert(id, type_);
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Whether we have a type.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn has(&self, type_: &TypeT) -> bool {
        for our_type in self.types.values() {
            if ptr::eq(*our_type, type_) {
                return true;
            }
        }
        false
    }

    /// Whether we have a type.
    pub fn has_id(&self, type_id: &ID) -> bool {
        self.types.contains_key(type_id)
    }

    /// The type.
    pub fn get_by_id(&self, type_id: &ID) -> Option<&'own TypeT> {
        self.types.get(type_id).map(|type_| *type_)
    }

    // /// The type.
    // pub fn find(&self, type_id: &ID, namespace: &Namespace) -> Option<(ID, &'own TypeT)> {
    //     let type_id = type_id.in_namespace(namespace.clone());
    //     self.get_by_name(&type_id).map(|type_| (type_id, type_))
    // }

    /// The complete type.
    pub fn get_complete(&self, type_id: &ID) -> Option<&TypeT> {
        self.complete.get(type_id)
    }

    /// The [ID] of the type.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_id(&self, type_: &TypeT) -> Option<&ID> {
        for (our_type_id, our_type) in &self.types {
            if ptr::eq(*our_type, type_) {
                return Some(our_type_id);
            }
        }
        None
    }

    /// The parent of the type.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_parent(&self, type_: &TypeT) -> Option<&TypeT> {
        self.get_id(type_).and_then(|id| self.get_parent_by_id(id))
    }

    /// The parent of the type.
    pub fn get_parent_by_id(&self, type_id: &ID) -> Option<&TypeT> {
        self.parents.get(type_id).map(|type_| *type_)
    }

    /// The [ID] of the type's parent if it has a parent.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_parent_id(&self, type_: &TypeT) -> Option<&ID> {
        self.get_id(type_).and_then(|id| self.get_parent_id_by_id(id))
    }

    /// The [ID] of the type's parent if it has a parent.
    pub fn get_parent_id_by_id(&self, type_id: &ID) -> Option<&ID> {
        self.parents.get(type_id).and_then(|type_| self.get_id(type_))
    }

    /// The type's children.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_children(&self, type_: &TypeT) -> Option<Vec<&TypeT>> {
        if !self.has(type_) {
            return None;
        }

        let mut child_types = Vec::default();
        for (child_type_id, parent_type) in &self.parents {
            if ptr::eq(*parent_type, type_) {
                if let Some(child_type) = self.get_by_id(child_type_id) {
                    child_types.push(child_type);
                }
            }
        }
        Some(child_types)
    }

    /// The type's children.
    pub fn get_children_by_id(&self, type_id: &ID) -> Option<Vec<&TypeT>> {
        self.get_by_id(type_id).and_then(|type_| self.get_children(type_))
    }

    /// The [ID]s of the type's children.
    ///
    /// Important: The identity of the entity is the *pointer* represented by the reference.
    /// Thus a clone of a value or an otherwise equal value will *not* be considered identical.
    pub fn get_children_ids(&self, type_: &TypeT) -> Option<Vec<&ID>> {
        if !self.has(type_) {
            return None;
        }

        let mut child_type_ids = Vec::default();
        for (child_type_id, parent_type) in &self.parents {
            if ptr::eq(*parent_type, type_) {
                child_type_ids.push(child_type_id);
            }
        }
        Some(child_type_ids)
    }

    /// The [ID]s of the type's children.
    pub fn get_children_ids_by_id(&self, type_id: &ID) -> Option<Vec<&ID>> {
        self.get_by_id(type_id).and_then(|type_| self.get_children_ids(type_))
    }

    /// The root types (types without parents).
    pub fn get_roots(&self) -> Vec<&TypeT> {
        let mut root_types = Vec::default();
        for (type_id, type_) in &self.types {
            if !self.parents.contains_key(type_id) {
                root_types.push(*type_)
            }
        }
        root_types
    }

    /// The [ID]s of the root types (types without parents).
    pub fn get_root_ids(&self) -> Vec<&ID> {
        let mut root_type_ids = Vec::default();
        for type_id in self.types.keys() {
            if !self.parents.contains_key(type_id) {
                root_type_ids.push(type_id)
            }
        }
        root_type_ids
    }

    /// True if the type is a descendent of the ancestor.
    pub fn is_descendent(&self, ancestor_type: &TypeT, type_: &TypeT) -> bool {
        if let Some(parent_type) = self.get_parent(type_) {
            if ptr::eq(ancestor_type, parent_type) {
                return true;
            }
            if self.is_descendent(ancestor_type, parent_type) {
                return true;
            }
        }
        false
    }

    /// Populate parents.
    pub fn populate<ErrorRecipientT>(
        &mut self,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        for (type_id, type_) in &self.types {
            if let Some(parent_type_name) = type_.get_parent_name() {
                match index.index.get(parent_type_name) {
                    Some(parent_type_id) => {
                        match self.get_by_id(parent_type_id) {
                            Some(parent_type) => {
                                if ptr::eq(parent_type, *type_) {
                                    // Derived from self
                                    errors.give(type_.with_parent_name_field_annotations(
                                        CyclicalDerivationError::new(parent_type_id.to_string()),
                                    ))?;
                                } else {
                                    self.parents.insert(type_id.clone(), parent_type);
                                }
                            }

                            None => errors.give(type_.with_parent_name_field_annotations(
                                UnknownTypeError::new(parent_type_id.to_string(), "types.populate".into()),
                            ))?,
                        }
                    }

                    None => errors.give(type_.with_parent_name_field_annotations(UnknownTypeError::new(
                        parent_type_name.to_string(),
                        "types.populate".into(),
                    )))?,
                }
            }
        }

        // Check for inheritance loops
        for (type_id, type_) in &self.types {
            if self.is_descendent(type_, type_) {
                let parent_type = self.parents.remove(type_id).expect("remove");
                let parent_type_id = self.get_id(parent_type).expect("get_name");
                errors.give(
                    type_.with_parent_name_field_annotations(CyclicalDerivationError::new(parent_type_id.to_string())),
                )?;
            }
        }

        Ok(())
    }

    /// Complete all types.
    pub fn complete<ErrorRecipientT>(
        &self,
        catalog: &CatalogT,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<FastHashMap<ID, TypeT>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_types = FastHashMap::default();

        let type_ids: Vec<_> = self.types.keys().cloned().collect();
        for type_id in type_ids {
            TypeT::complete_into(type_id, &mut complete_types, self, catalog, index, errors)?;
        }

        Ok(complete_types)
    }

    /// Compile to Floria.
    pub fn compile_to_floria<StoreT, ErrorRecipientT>(
        &self,
        store: &StoreT,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        for (type_id, type_) in &self.complete {
            let parent_type_id = self.get_parent_id_by_id(type_id);
            type_.compile(type_id, parent_type_id, store, errors)?;
        }
        Ok(())
    }

    /// Add our and our ancestors' Floria group IDs.
    pub fn add_floria_group_ids(&self, group_ids: &mut Vec<floria::ID>, prefix: &ByteString, id: &ID) {
        match self.get_by_id(id) {
            Some(type_) => {
                group_ids.push(id.to_group_id(prefix.clone()));

                if let Some(parent_type_id) = self.get_parent_id(type_) {
                    self.add_floria_group_ids(group_ids, prefix, parent_type_id);
                }
            }

            None => tracing::warn!("add_floria_group_ids: type not found: {}", id),
        }
    }

    fn write_debug_for_names<WriteT>(
        &self,
        mut type_ids: Vec<&ID>,
        writer: &mut WriteT,
        context: &DebugContext,
    ) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        type_ids.sort();

        for (type_id, last) in IterateWithLast::new(type_ids) {
            context.indent_into_branch(writer, last)?;
            context.theme.write_name(writer, type_id)?;

            if let Some(child_type_ids) = self.get_children_ids_by_id(type_id) {
                self.write_debug_for_names(child_type_ids, writer, &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    }

    fn write_debug_for_names_with_templates<TemplateEntityT, WriteT>(
        &self,
        type_ids: Vec<&ID>,
        templates: &Templates<'own, TemplateEntityT, TypeT, CatalogT, AnnotatedT>,
        templates_heading: &str,
        writer: &mut WriteT,
        context: &DebugContext,
    ) -> io::Result<()>
    where
        TemplateEntityT: Template<TypeT, CatalogT, AnnotatedT>,
        WriteT: io::Write,
    {
        for (type_id, last) in IterateWithLast::new(type_ids) {
            context.indent_into_branch(writer, last)?;
            context.theme.write_name(writer, type_id)?;

            if let Some(child_type_ids) = self.get_children_ids_by_id(type_id) {
                self.write_debug_for_names_with_templates(
                    child_type_ids,
                    templates,
                    templates_heading,
                    writer,
                    &context.child().increase_indentation_branch(last),
                )?;
            }

            let template_names = templates.get_template_ids_by_type_id(type_id);
            if !template_names.is_empty() {
                let item_context = context.child().increase_indentation_branch(last);
                item_context.indent(writer)?;
                context.theme.write_meta(writer, format!("{}:", templates_heading))?;

                for (name, last) in IterateWithLast::new(template_names) {
                    item_context.indent_into_double_branch(writer, last)?;
                    context.theme.write_string(writer, name)?;
                }
            }
        }

        Ok(())
    }

    fn write_debug_with_templates<TemplateEntityT, WriteT>(
        &self,
        templates: &Templates<'own, TemplateEntityT, TypeT, CatalogT, AnnotatedT>,
        templates_heading: &str,
        writer: &mut WriteT,
        context: &DebugContext,
    ) -> io::Result<()>
    where
        TemplateEntityT: Template<TypeT, CatalogT, AnnotatedT>,
        WriteT: io::Write,
    {
        self.write_debug_for_names_with_templates(self.get_root_ids(), templates, templates_heading, writer, context)
    }
}

impl<'own, TypeT, CatalogT, AnnotatedT> Default for Types<'own, TypeT, CatalogT, AnnotatedT> {
    fn default() -> Self {
        Self {
            types: Default::default(),
            parents: Default::default(),
            complete: Default::default(),
            catalog: PhantomData,
            annotated: PhantomData,
        }
    }
}

impl<'own, TypeT, CatalogT, AnnotatedT> Debuggable for Types<'own, TypeT, CatalogT, AnnotatedT>
where
    TypeT: Type<CatalogT, AnnotatedT>,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        self.write_debug_for_names(self.get_root_ids(), writer, context)
    }
}

//
// TypesWithTemplates
//

/// [Types] with [Templates].
pub struct TypesWithTemplates<'own, TypeT, TemplateT, CatalogT, AnnotatedT> {
    /// Types.
    pub types: &'own Types<'own, TypeT, CatalogT, AnnotatedT>,

    /// Templates.
    pub templates: &'own Templates<'own, TemplateT, TypeT, CatalogT, AnnotatedT>,

    /// Templates heading.
    pub templates_heading: &'own str,
}

impl<'own, TypeT, CatalogT, AnnotatedT> Types<'own, TypeT, CatalogT, AnnotatedT> {
    /// Constructor.
    pub fn with_templates<TemplateEntityT>(
        &'own self,
        templates: &'own Templates<'own, TemplateEntityT, TypeT, CatalogT, AnnotatedT>,
        templates_heading: &'own str,
    ) -> TypesWithTemplates<'own, TypeT, TemplateEntityT, CatalogT, AnnotatedT>
    where
        TypeT: Type<CatalogT, AnnotatedT>,
        TemplateEntityT: Template<TypeT, CatalogT, AnnotatedT>,
    {
        TypesWithTemplates { types: self, templates, templates_heading }
    }
}

impl<'own, TypeT, TemplateT, CatalogT, AnnotatedT> Debuggable
    for TypesWithTemplates<'own, TypeT, TemplateT, CatalogT, AnnotatedT>
where
    TypeT: Type<CatalogT, AnnotatedT>,
    TemplateT: Template<TypeT, CatalogT, AnnotatedT>,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        self.types.write_debug_with_templates(self.templates, self.templates_heading, writer, context)
    }
}
