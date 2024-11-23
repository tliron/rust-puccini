use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::std::error::*,
    std::{slice, vec},
};

//
// TaggedValues
//

/// A sequence of tag-value pairs.
///
/// A tag-value pair is a bit like a key-value pair in a map, except that in a map the keys are
/// unique but tags do not have that requirement, i.e. multiple values may have the same tag. Thus
/// the only requirement for tags is that they implement [PartialEq].
///
/// It can be resolved from a [List] in which each item is a [Map] with a single key, which is the
/// tag for the value. In TOSCA this notation is sometimes called "sequenced list".
#[derive(Clone, Debug)]
pub struct TaggedValues<TagT, ValueT>(pub Vec<(TagT, ValueT)>);

impl<TagT, ValueT> TaggedValues<TagT, ValueT> {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add a tag-value pair.
    pub fn add(&mut self, tag: TagT, value: ValueT) {
        self.0.push((tag, value))
    }
}

impl<TagT, ValueT> TaggedValues<TagT, ValueT>
where
    TagT: PartialEq,
{
    /// True if contains the tag at least once.
    pub fn contains_tag(&self, tag: &TagT) -> bool {
        for (our_tag, _) in &self.0 {
            if our_tag == tag {
                return true;
            }
        }
        false
    }

    /// Gets the value of the first occurrence of the tag.
    pub fn get_first(&self, tag: &TagT) -> Option<&ValueT> {
        for (our_tag, value) in &self.0 {
            if our_tag == tag {
                return Some(value);
            }
        }
        None
    }

    /// Gets the value of the first occurrence of the tag.
    pub fn get_first_mut(&mut self, tag: &TagT) -> Option<&mut ValueT> {
        for (our_tag, value) in &mut self.0 {
            if our_tag == tag {
                return Some(value);
            }
        }
        None
    }
}

impl<TagT, ValueT, AnnotatedT> Resolve<TaggedValues<TagT, ValueT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
    Variant<AnnotatedT>: Resolve<TagT, AnnotatedT> + Resolve<ValueT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<TaggedValues<TagT, ValueT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved = Vec::default();

        // List of single-key maps
        match self {
            Self::List(list) => {
                for item in list {
                    match item {
                        Self::Map(map) => {
                            if map.inner.len() == 1 {
                                let pair = map.into_key_value_pair().expect("single-key map");
                                if let Some((key, value)) = pair.resolve_with_errors(errors)? {
                                    resolved.push((key, value));
                                }
                            } else {
                                errors.give(
                                    MalformedError::new(
                                        "tag-value pair".into(),
                                        "is not a map with a single key".into(),
                                    )
                                    .with_annotations_from(&map),
                                )?;
                            }
                        }

                        _ => errors.give(IncompatibleVariantTypeError::new_from(&item, &["map"]))?,
                    }
                }
            }

            _ => errors.give(IncompatibleVariantTypeError::new_from(&self, &["list"]))?,
        }

        Ok(Some(TaggedValues(resolved)))
    }
}

impl<TagT, ValueT> Default for TaggedValues<TagT, ValueT> {
    fn default() -> Self {
        Self(Default::default())
    }
}

// Delegated

impl<TagT, ValueT> IntoIterator for TaggedValues<TagT, ValueT> {
    type Item = (TagT, ValueT);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'own, TagT, ValueT> IntoIterator for &'own TaggedValues<TagT, ValueT> {
    type Item = &'own (TagT, ValueT);
    type IntoIter = slice::Iter<'own, (TagT, ValueT)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'own, TagT, ValueT> IntoIterator for &'own mut TaggedValues<TagT, ValueT> {
    type Item = &'own mut (TagT, ValueT);
    type IntoIter = slice::IterMut<'own, (TagT, ValueT)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// Conversions

impl<TagT, ValueT> From<TaggedValues<TagT, ValueT>> for Vec<(TagT, ValueT)> {
    fn from(value: TaggedValues<TagT, ValueT>) -> Self {
        value.0
    }
}
