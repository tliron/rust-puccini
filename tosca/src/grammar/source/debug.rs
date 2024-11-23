use super::{super::entity::*, source::*};

use {
    kutil::{cli::debug::*, std::iter::*},
    std::io,
};

const PREFIX: char = '@';

impl Source {
    /// To [DebuggableNamespaces].
    pub fn to_debuggable_namespaces<'own>(&'own self, entity_kinds: &'own EntityKinds) -> DebuggableNamespaces<'own> {
        DebuggableNamespaces::new(self, entity_kinds)
    }

    /// To [DebuggableEntities].
    pub fn to_debuggable_entities<'own>(&'own self, entity_kinds: &'own EntityKinds) -> DebuggableEntities<'own> {
        DebuggableEntities::new(self, entity_kinds)
    }
}

//
// DebuggableNamespaces
//

/// Debuggable namespaces.
pub struct DebuggableNamespaces<'own> {
    /// Source.
    pub source: &'own Source,

    /// Entity kinds.
    pub entity_kinds: &'own EntityKinds,
}

impl<'own> DebuggableNamespaces<'own> {
    /// Constructor.
    pub fn new(source: &'own Source, entity_kinds: &'own EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'own> Debuggable for DebuggableNamespaces<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for ((entity_kind, names), last) in IterateWithLast::new(self.source.namespace_tree()) {
            let entity_kind_name = self.entity_kinds.represent(entity_kind);
            context.indent_into_branch(writer, last)?;
            context.theme.write_heading(writer, entity_kind_name)?;

            let context = context.child().increase_indentation_branch(last);
            for ((full_name, source_id), last) in IterateWithLast::new(names) {
                context.indent_into_branch(writer, last)?;
                write!(
                    writer,
                    "{} {}{}",
                    context.theme.name(full_name),
                    context.theme.delimiter(PREFIX),
                    context.theme.meta(source_id)
                )?;
            }
        }

        Ok(())
    }
}

//
// DebuggableEntities
//

/// Debuggable entities.
pub struct DebuggableEntities<'own> {
    /// Source.
    pub source: &'own Source,

    /// Entity kinds.
    pub entity_kinds: &'own EntityKinds,
}

impl<'own> DebuggableEntities<'own> {
    /// Constructor.
    pub fn new(source: &'own Source, entity_kinds: &'own EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'own> Debuggable for DebuggableEntities<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for ((entity_kind, names), last) in IterateWithLast::new(self.source.entity_names_tree()) {
            let entity_kind_name = self.entity_kinds.represent(entity_kind);
            context.indent_into_branch(writer, last)?;
            context.theme.write_heading(writer, entity_kind_name)?;

            let context = context.child().increase_indentation_branch(last).with_configuration("heading", "false");
            for (name, last) in IterateWithLast::new(names) {
                let entity = self.source.entities.get(&WithEntityKind::new(entity_kind, name.clone())).expect("entity");

                context.indent_into_branch(writer, last)?;
                context.theme.write_name(writer, name)?;
                entity.dyn_write_debug_for(Box::new(writer), &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    }
}
