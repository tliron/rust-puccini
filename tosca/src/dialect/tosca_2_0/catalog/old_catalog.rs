use super::super::{
    super::super::grammar::*,
    entities::{File, *},
};

use {
    compris::annotate::*,
    kutil::{
        cli::debug::*,
        std::{error::*, zerocopy::*},
    },
    std::io,
};

//
// OldCatalog
//

/// Container for TOSCA types and templates.
#[derive(Debug)]
pub struct OldCatalog<'own, AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Artifact types.
    pub artifact_types: Types<'own, ArtifactType<AnnotatedT>, Self, AnnotatedT>,

    /// Capability types.
    pub capability_types: Types<'own, CapabilityType<AnnotatedT>, Self, AnnotatedT>,

    /// Data types.
    pub data_types: Types<'own, DataType<AnnotatedT>, Self, AnnotatedT>,

    /// Group types.
    pub group_types: Types<'own, GroupType<AnnotatedT>, Self, AnnotatedT>,

    /// Interface types.
    pub interface_types: Types<'own, InterfaceType<AnnotatedT>, Self, AnnotatedT>,

    /// Node types.
    pub node_types: Types<'own, NodeType<AnnotatedT>, Self, AnnotatedT>,

    /// Policy types.
    pub policy_types: Types<'own, PolicyType<AnnotatedT>, Self, AnnotatedT>,

    /// Relationship types.
    pub relationship_types: Types<'own, RelationshipType<AnnotatedT>, Self, AnnotatedT>,

    /// Node templates.
    pub node_templates: Templates<'own, NodeTemplate<AnnotatedT>, NodeType<AnnotatedT>, Self, AnnotatedT>,

    /// Relationship templates.
    pub relationship_templates:
        Templates<'own, RelationshipTemplate<AnnotatedT>, RelationshipType<AnnotatedT>, Self, AnnotatedT>,

    /// Group templates.
    pub group_templates: Templates<'own, GroupTemplate<AnnotatedT>, GroupType<AnnotatedT>, Self, AnnotatedT>,

    /// Policy templates.
    pub policy_templates: Templates<'own, PolicyTemplate<AnnotatedT>, PolicyType<AnnotatedT>, Self, AnnotatedT>,
}

impl<'own, AnnotatedT> OldCatalog<'own, AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_for_package<ErrorRecipientT>(
        package: &'own OldPackage<File<AnnotatedT>, Self, AnnotatedT>,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut catalog = OldCatalog::default();
        for packaged_file in &package.packaged_files {
            catalog.add_file(&packaged_file.file, &packaged_file.source);
        }
        catalog.populate(index, errors)?;
        catalog.complete(index, errors)?;
        return Ok(catalog);
    }

    /// Add a [File].
    pub fn add_file(&mut self, file: &'own File<AnnotatedT>, source: &ByteString) {
        tracing::info!("add_file: {}", source);

        let source = SourceID::URL(source.clone());

        self.artifact_types.add_all(&source, &file.artifact_types);
        self.capability_types.add_all(&source, &file.capability_types);
        self.data_types.add_all(&source, &file.data_types);
        self.group_types.add_all(&source, &file.group_types);
        self.interface_types.add_all(&source, &file.interface_types);
        self.node_types.add_all(&source, &file.node_types);
        self.policy_types.add_all(&source, &file.policy_types);
        self.relationship_types.add_all(&source, &file.relationship_types);
        self.artifact_types.add_all(&source, &file.artifact_types);

        if let Some(service_template) = &file.service_template {
            self.node_templates.add_all(&source, &service_template.node_templates);
            self.relationship_templates.add_all(&source, &service_template.relationship_templates);
            self.group_templates.add_all(&source, &service_template.groups);

            // self.policy_templates.add_all(
            //     &service_template.policy_templates,
            //     &namespace,
            //     &self.policy_types,
            //     errors,
            // )?;
        }
    }

    /// Populate.
    pub fn populate<ErrorRecipientT>(
        &mut self,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        self.artifact_types.populate(index, errors)?;
        self.capability_types.populate(index, errors)?;
        self.data_types.populate(index, errors)?;
        self.group_types.populate(index, errors)?;
        self.interface_types.populate(index, errors)?;
        self.node_types.populate(index, errors)?;
        self.policy_types.populate(index, errors)?;
        self.relationship_types.populate(index, errors)?;

        self.node_templates.populate(&self.node_types, index, errors)?;
        self.relationship_templates.populate(&self.relationship_types, index, errors)?;
        self.group_templates.populate(&self.group_types, index, errors)?;
        self.policy_templates.populate(&self.policy_types, index, errors)?;

        Ok(())
    }

    /// Complete.
    pub fn complete<ErrorRecipientT>(
        &mut self,
        index: &Index,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        self.artifact_types.complete = self.artifact_types.complete(self, index, errors)?;
        self.capability_types.complete = self.capability_types.complete(self, index, errors)?;
        self.data_types.complete = self.data_types.complete(self, index, errors)?;
        self.group_types.complete = self.group_types.complete(self, index, errors)?;
        self.interface_types.complete = self.interface_types.complete(self, index, errors)?;
        self.node_types.complete = self.node_types.complete(self, index, errors)?;
        self.policy_types.complete = self.policy_types.complete(self, index, errors)?;
        self.relationship_types.complete = self.relationship_types.complete(self, index, errors)?;

        self.node_templates.complete = self.node_templates.complete(&self.node_types, self, errors)?;
        self.relationship_templates.complete =
            self.relationship_templates.complete(&self.relationship_types, self, errors)?;
        self.group_templates.complete = self.group_templates.complete(&self.group_types, self, errors)?;
        self.policy_templates.complete = self.policy_templates.complete(&self.policy_types, self, errors)?;

        Ok(())
    }

    /// Compile types to Floria.
    pub fn compile_types_to_floria<StoreT, ErrorRecipientT>(
        &self,
        store: &StoreT,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        self.artifact_types.compile_to_floria(store, errors)?;
        self.capability_types.compile_to_floria(store, errors)?;
        self.data_types.compile_to_floria(store, errors)?;
        self.group_types.compile_to_floria(store, errors)?;
        self.interface_types.compile_to_floria(store, errors)?;
        self.node_types.compile_to_floria(store, errors)?;
        self.policy_types.compile_to_floria(store, errors)?;
        self.relationship_types.compile_to_floria(store, errors)?;

        Ok(())
    }
}

impl<'own, AnnotatedT> Default for OldCatalog<'own, AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn default() -> Self {
        Self {
            artifact_types: Default::default(),
            capability_types: Default::default(),
            data_types: Default::default(),
            group_types: Default::default(),
            interface_types: Default::default(),
            node_types: Default::default(),
            policy_types: Default::default(),
            relationship_types: Default::default(),
            node_templates: Default::default(),
            relationship_templates: Default::default(),
            group_templates: Default::default(),
            policy_templates: Default::default(),
        }
    }
}

impl<'own, AnnotatedT> Debuggable for OldCatalog<'own, AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_heading(writer, "Types")?;

        let item_context = &context.child().increase_indentation();

        if !self.artifact_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "artifact types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.artifact_types.write_debug_for(writer, item_context)?;
        }

        if !self.capability_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "capability types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.capability_types.write_debug_for(writer, item_context)?;
        }

        if !self.data_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "data types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.data_types.write_debug_for(writer, item_context)?;
        }

        if !self.group_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "group types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.group_types.with_templates(&self.group_templates, "groups").write_debug_for(writer, item_context)?;
        }

        if !self.interface_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "interface types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.interface_types.write_debug_for(writer, item_context)?;
        }

        if !self.node_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "node types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.node_types
                .with_templates(&self.node_templates, "node templates")
                .write_debug_for(writer, item_context)?;
        }

        if !self.policy_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "policy types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.policy_types
                .with_templates(&self.policy_templates, "policies")
                .write_debug_for(writer, item_context)?;
        }

        if !self.relationship_types.is_empty() {
            item_context.indent(writer)?;
            item_context.theme.write_meta(writer, "relationship types")?;
            item_context.theme.write_delimiter(writer, ":")?;
            self.relationship_types
                .with_templates(&self.relationship_templates, "relationship templates")
                .write_debug_for(writer, item_context)?;
        }

        Ok(())
    }
}
