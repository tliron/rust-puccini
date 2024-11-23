use super::{super::entity::*, depot::*};

use {
    compris::annotate::*,
    kutil::{cli::debug::*, std::iter::*},
    std::io,
};

impl Depot {
    /// To [DebuggableNamespaces].
    pub fn to_debuggable_namespaces<'own>(&'own self) -> DebuggableNamespaces<'own> {
        DebuggableNamespaces::new(self)
    }

    /// To [DebuggableEntities].
    pub fn to_debuggable_entities<'own>(&'own self) -> DebuggableEntities<'own> {
        DebuggableEntities::new(self)
    }
}

//
// DebuggableNamespaces
//

/// Debuggable namespaces.
pub struct DebuggableNamespaces<'own> {
    /// Depot.
    pub depot: &'own Depot,
}

impl<'own> DebuggableNamespaces<'own> {
    /// Constructor.
    pub fn new(depot: &'own Depot) -> Self {
        Self { depot }
    }
}

impl<'own> Debuggable for DebuggableNamespaces<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let default_entity_kinds = EntityKinds::default();

        for ((source_id, source), first) in IterateWithFirst::new(IterateByKeyOrder::new(&self.depot.sources)) {
            let entity_kinds = self
                .depot
                .dialect_entity_kinds::<WithoutAnnotations>(&source.dialect_id)
                .unwrap_or(&default_entity_kinds);

            if !first {
                writeln!(writer)?;
                context.indent(writer)?;
            }

            context.theme.write_heading(writer, source_id)?;
            source.to_debuggable_namespaces(entity_kinds).write_debug_for(writer, context)?;
        }

        Ok(())
    }
}

//
// DebuggableEntities
//

/// Debuggable entities.
pub struct DebuggableEntities<'own> {
    /// Depot.
    pub depot: &'own Depot,
}

impl<'own> DebuggableEntities<'own> {
    /// Constructor.
    pub fn new(depot: &'own Depot) -> Self {
        Self { depot }
    }
}

impl<'own> Debuggable for DebuggableEntities<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let default_entity_kinds = EntityKinds::default();

        for ((source_id, source), first) in IterateWithFirst::new(IterateByKeyOrder::new(&self.depot.sources)) {
            let entity_kinds = self
                .depot
                .dialect_entity_kinds::<WithoutAnnotations>(&source.dialect_id)
                .unwrap_or(&default_entity_kinds);

            if !first {
                writeln!(writer)?;
                context.indent(writer)?;
            }

            context.theme.write_heading(writer, source_id)?;
            source.to_debuggable_entities(entity_kinds).write_debug_for(writer, context)?;
        }

        Ok(())
    }
}
