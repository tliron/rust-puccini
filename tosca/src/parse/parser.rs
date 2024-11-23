use super::{super::grammar::*, errors::*, file::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::std::{collections::*, error::*, zerocopy::*},
    read_url::*,
    std::{io, sync::*},
};

// * bare name: just a string
// * local name: namespace + bare name (relative to one source)
// * global name: namespace + bare name (relative to package's root source)
// * canonical name: source id + bare name

// Source:
//  Box<dyn>
//
//  grammar: string
//  entities: Map (entity_kind, bare_name) => entity
//
//  Can be incomplete/complete (all entities)
//
//   Methods:
//     get_importables()
//       Importable -> get_source_id()
//     complete(&Depot)
//
//  Contains map:
//    Namespace -> source_id
//
//  find(ScopedName) -> CanonicalName
//    finds the namespace in list of namespaces
//    finds the source_id

// Catalog trait
//  get_source(source_id)
//  get_entity(canonical_name)
//    get_source(source_id).get_entity(bare_name)

// Depot: Catalog that stores complete Sources
//  Only complete Sources can be added!

// Package: Catalog that stores &Source
// It's a "view" of Depot

// Create a Package:
//  Start with the source_id (convert to URL)
//  If source_id is not already in Depot:
//   1) Read it from URL
//   2) Parse it into Variant
//   3) Find grammar from "tosca_definitions_version"
//   4) Resolve it into Source for that grammar
//   5) Get importables from Source
//   6) If not empty, recurse to 1) for each importable
//   7) Complete the Source!
//   7) Store in Depot: SourceID -> File
//   9) Store in Package: (SourceID, Namespace)
//  Depot now has all package's complete Files (as well as from other packages)

// How to complete an entity:
//   complete(source_id, &Depot)
//     get parent ScopedName
//     get &Source from package
//     canonical_name = Source.find(ScopedName)
//     parent = package.get_entity(canonical_name)

/// [Parser] cache.
pub type ParserCache<AnnotatedT> = Mutex<FastHashMap<ByteString, Variant<AnnotatedT>>>;

/// Common reference type for [ParserCache].
pub type ParserCacheRef<AnnotatedT> = Arc<ParserCache<AnnotatedT>>;

//
// Parser
//

/// Puccini Parser.
#[allow(dead_code)]
pub struct Parser<AnnotatedT> {
    url_context: UrlContextRef,
    cache: ParserCacheRef<AnnotatedT>,
}

impl<AnnotatedT> Parser<AnnotatedT> {
    /// Constructor.
    pub fn new(url_context: UrlContextRef) -> Self {
        Self::new_with_cache(url_context, ParserCache::default().into())
    }

    /// Constructor.
    pub fn new_with_cache(url_context: UrlContextRef, cache: ParserCacheRef<AnnotatedT>) -> Self {
        Self { url_context, cache }
    }

    /// Constructor.
    pub fn new_child(&self, url_context: UrlContextRef) -> Self {
        Self::new_with_cache(url_context, self.cache.clone())
    }

    /// Parse into a [Package] from path, URL, or stdin.
    pub fn parse<FileT, CatalogT, ErrorRecipientT>(
        &self,
        input_path_or_url: Option<ByteString>,
        resolve_errors: &mut ErrorRecipientT,
    ) -> Result<OldPackage<FileT, CatalogT, AnnotatedT>, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        FileT: File<CatalogT, AnnotatedT>,
        Variant<AnnotatedT>: Resolve<FileT, AnnotatedT>,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        self.resolve_variant_with_imports(&self.get_variant(input_path_or_url, Default::default())?, resolve_errors)
    }

    /// Get [FileVariant] by path, URL, or stdin.
    fn get_variant(
        &self,
        input_path_or_url: Option<ByteString>,
        scope: Scope,
    ) -> Result<FileVariant<AnnotatedT>, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match input_path_or_url {
            Some(input_path_or_url) => {
                let url = self.url_context.url_or_file_path(&input_path_or_url)?;
                self.get_variant_from_cache_or_url(url, scope)
            }

            None => FileVariant::read(&mut io::stdin(), Default::default(), scope),
        }
    }

    /// Get [FileVariant] from cache or [URL].
    fn get_variant_from_cache_or_url(
        &self,
        url: UrlRef,
        scope: Scope,
    ) -> Result<FileVariant<AnnotatedT>, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let source: ByteString = url.to_string().into();
        let source_id = SourceID::URL(source.clone());

        let variant = {
            let cache = self.cache.lock()?;
            cache.get(&source).cloned()
        };

        match variant {
            Some(variant) => Ok(FileVariant::new(variant, source_id, scope)),

            None => {
                let mut reader = url.open()?;
                let file_variant = FileVariant::read(&mut reader, source_id, scope)?;

                let mut cache = self.cache.lock()?;
                cache.insert(source, file_variant.variant.clone());

                Ok(file_variant)
            }
        }
    }

    /// Resolve a [FileVariant] into a [Package].
    ///
    /// Will recursively resolve imports.
    fn resolve_variant_with_imports<FileT, CatalogT, ErrorRecipientT>(
        &self,
        file_variant: &FileVariant<AnnotatedT>,
        resolve_errors: &mut ErrorRecipientT,
    ) -> Result<OldPackage<FileT, CatalogT, AnnotatedT>, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        FileT: File<CatalogT, AnnotatedT>,
        Variant<AnnotatedT>: Resolve<FileT, AnnotatedT>,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut package = OldPackage::default();

        if let Some(file) = file_variant.variant.resolve_with_errors(resolve_errors)? {
            let file_variants = self.get_imported_variants(&file, file_variant)?;
            package.add_file(file, file_variant.source_id.clone().into());
            for file_variant in &file_variants {
                let imported_package = self.resolve_variant_with_imports(file_variant, resolve_errors)?;
                package.merge(imported_package);
            }
        }

        Ok(package)
    }

    /// Get imported [FileVariant]s.
    fn get_imported_variants<'own, FileT, CatalogT>(
        &self,
        file: &'own FileT,
        file_variant: &FileVariant<AnnotatedT>,
    ) -> Result<Vec<FileVariant<AnnotatedT>>, PucciniError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        FileT: File<CatalogT, AnnotatedT>,
    {
        let url_context = self.get_url_context_for(&file_variant.source_id.to_string())?;

        let importables = file.get_importables();
        let mut file_variants = Vec::with_capacity(importables.len());

        for importable in importables {
            let namespace = file_variant.scope.clone();
            // if let Some(import_namespace) = importable.namespace {
            //     namespace.0.extend_from_slice(&import_namespace);
            // }

            let parser = self.new_child(url_context.clone());
            let file_variant = parser.get_variant(Some(importable.url), namespace)?;

            file_variants.push(file_variant);
        }

        Ok(file_variants)
    }

    /// URL context.
    fn get_url_context_for(&self, source: &str) -> Result<UrlContextRef, PucciniError<AnnotatedT>> {
        if !source.is_empty() {
            let url = self.url_context.url_or_file_path(source)?;
            if let Some(base) = url.base() {
                let mut base_urls = self.url_context.clone_base_urls();
                base_urls.insert(0, base.into());
                return Ok(self.url_context.with_base_urls(base_urls));
            }
        }

        Ok(self.url_context.clone())
    }
}
