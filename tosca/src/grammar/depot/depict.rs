use super::{super::entity::*, depot::*};

use {
    compris::annotate::*,
    kutil::{cli::depict::*, std::iter::*},
    std::io,
};

impl Depot {
    /// To [NamespacesDepiction].
    pub fn namespaces_depiction<'own>(&'own self) -> NamespacesDepiction<'own> {
        NamespacesDepiction::new(self)
    }

    /// To [EntitiesDepiction].
    pub fn entities_depiction<'own>(&'own self) -> EntitiesDepiction<'own> {
        EntitiesDepiction::new(self)
    }
}

//
// NamespacesDepiction
//

/// Namespaces depiction.
pub struct NamespacesDepiction<'own> {
    /// Depot.
    pub depot: &'own Depot,
}

impl<'own> NamespacesDepiction<'own> {
    /// Constructor.
    pub fn new(depot: &'own Depot) -> Self {
        Self { depot }
    }
}

impl<'own> Depict for NamespacesDepiction<'own> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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
            source.namespaces_depiction(entity_kinds).depict(writer, context)?;
        }

        Ok(())
    }
}

//
// EntitiesDepiction
//

/// Entities depiction.
pub struct EntitiesDepiction<'own> {
    /// Depot.
    pub depot: &'own Depot,
}

impl<'own> EntitiesDepiction<'own> {
    /// Constructor.
    pub fn new(depot: &'own Depot) -> Self {
        Self { depot }
    }
}

impl<'own> Depict for EntitiesDepiction<'own> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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
            source.entities_depiction(entity_kinds).depict(writer, context)?;
        }

        Ok(())
    }
}
