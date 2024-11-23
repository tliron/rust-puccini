use super::{super::entity::*, source::*};

use {
    kutil::{cli::depict::*, std::iter::*},
    std::io,
};

const PREFIX: char = '@';

impl Source {
    /// To [NamespacesDepiction].
    pub fn namespaces_depiction<'own>(&'own self, entity_kinds: &'own EntityKinds) -> NamespacesDepiction<'own> {
        NamespacesDepiction::new(self, entity_kinds)
    }

    /// To [EntitiesDepiction].
    pub fn entities_depiction<'own>(&'own self, entity_kinds: &'own EntityKinds) -> EntitiesDepiction<'own> {
        EntitiesDepiction::new(self, entity_kinds)
    }
}

//
// NamespacesDepiction
//

/// Namespaces depiction.
pub struct NamespacesDepiction<'own> {
    /// Source.
    pub source: &'own Source,

    /// Entity kinds.
    pub entity_kinds: &'own EntityKinds,
}

impl<'own> NamespacesDepiction<'own> {
    /// Constructor.
    pub fn new(source: &'own Source, entity_kinds: &'own EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'own> Depict for NamespacesDepiction<'own> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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
// EntitiesDepiction
//

/// Entities depiction.
pub struct EntitiesDepiction<'own> {
    /// Source.
    pub source: &'own Source,

    /// Entity kinds.
    pub entity_kinds: &'own EntityKinds,
}

impl<'own> EntitiesDepiction<'own> {
    /// Constructor.
    pub fn new(source: &'own Source, entity_kinds: &'own EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'own> Depict for EntitiesDepiction<'own> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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
                entity.dyn_depict(Box::new(writer), &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    }
}
