use super::{errors::*, old_entities::*};

use {
    kutil::std::{error::*, zerocopy::*},
    std::marker::*,
};

//
// PackagedFile
//

/// [File] in a [Package].
#[derive(Clone, Debug)]
pub struct PackagedFile<FileT> {
    /// File.
    pub file: FileT,

    /// Source.
    pub source: ByteString,
}

impl<FileT> PackagedFile<FileT> {
    /// Constructor.
    pub fn new(file: FileT, source: ByteString) -> Self {
        Self { file, source }
    }
}

//
// OldPackage
//

/// A package is a collection of [File]s. It can represent either a service template or a profile.
pub struct OldPackage<FileT, CatalogT, AnnotatedT>
where
    FileT: File<CatalogT, AnnotatedT>,
{
    /// Packaged files.
    pub packaged_files: Vec<PackagedFile<FileT>>,

    catalog: PhantomData<CatalogT>,
    annotated: PhantomData<AnnotatedT>,
}

impl<FileT, CatalogT, AnnotatedT> OldPackage<FileT, CatalogT, AnnotatedT>
where
    FileT: File<CatalogT, AnnotatedT>,
{
    /// Root.
    ///
    /// This is the first file added to the package.
    pub fn root(&self) -> Option<&PackagedFile<FileT>> {
        self.packaged_files.get(0)
    }

    /// Add file.
    ///
    /// The first file added will be considered the root.
    pub fn add_file(&mut self, file: FileT, source: ByteString) {
        self.packaged_files.push(PackagedFile::new(file, source));
    }

    /// Merge.
    pub fn merge(&mut self, other: Self) {
        self.packaged_files.extend(other.packaged_files);
    }

    /// Compile to Floria.
    pub fn compile_to_floria<StoreT, ErrorRecipientT>(
        &self,
        context: CompileToFloriaContext<CatalogT, StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        StoreT: Clone + floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        match self.root() {
            Some(file_entry) => file_entry.file.compile_to_floria(context.clone(), file_entry.source.clone(), errors),

            None => Ok(None),
        }
    }
}

impl<FileT, CatalogT, AnnotatedT> Default for OldPackage<FileT, CatalogT, AnnotatedT>
where
    FileT: File<CatalogT, AnnotatedT>,
{
    fn default() -> Self {
        Self { packaged_files: Default::default(), catalog: PhantomData, annotated: PhantomData }
    }
}
