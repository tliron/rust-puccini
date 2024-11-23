use super::{
    super::{super::super::grammar::*, catalog::OldCatalog},
    attribute_definition::*,
    parameter_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::debug::*,
        std::{error::*, iter::*},
    },
    std::{fmt, io},
};

/// Function prefix.
pub const FUNCTION_PREFIX: &str = "$";

//
// ValueAssignment
//

/// Value assignment.
#[derive(Clone, Debug)]
pub enum ValueAssignment<AnnotatedT> {
    /// Literal.
    Literal(Variant<AnnotatedT>),

    /// Call.
    Call(Text<AnnotatedT>, Vec<ValueAssignment<AnnotatedT>>),
}

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// True if undefined.
    pub fn is_undefined(&self) -> bool {
        match self {
            Self::Literal(literal) => literal.is_undefined(),
            Self::Call(_, _) => true,
        }
    }

    /// Compile to Floria as property.
    pub fn compile_to_floria_as_property<ErrorRecipientT>(
        &self,
        property_definition: &PropertyDefinition<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        _errors: &mut ErrorRecipientT,
    ) -> Result<floria::Property, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let (variant, updater) = match self {
            Self::Literal(literal) => (Some(literal.clone().into_annotated()), None),
            Self::Call(name, _arguments) => (None, Some(floria::Call::new(name.clone().into(), Default::default()))),
        };

        let validator = match &property_definition.validation {
            Some(validation) => match validation {
                Self::Literal(_) => None, // TODO?
                Self::Call(name, _arguments) => Some(floria::Call::new(name.clone().into(), Default::default())),
            },
            None => None,
        };

        let mut floria_property = floria::Property::new(variant, updater, validator, true);
        floria_property.metadata.set_tosca_entity("Property");
        floria_property.metadata.set_tosca_description(property_definition.description.as_ref());
        floria_property.metadata.merge_tosca_metadata(&property_definition.metadata);

        catalog.data_types.add_floria_group_ids(
            &mut floria_property.group_ids,
            &"data".into(),
            index.index.get(&property_definition.type_name).unwrap(),
        );

        Ok(floria_property)
    }

    /// Compile to Floria as attribute.
    pub fn compile_to_floria_as_attribute<ErrorRecipientT>(
        &self,
        attribute_definition: &AttributeDefinition<AnnotatedT>,
        catalog: &OldCatalog<'_, AnnotatedT>,
        index: &Index,
        _errors: &mut ErrorRecipientT,
    ) -> Result<floria::Property, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let (variant, updater) = match self {
            Self::Literal(literal) => (Some(literal.clone().into_annotated()), None),
            Self::Call(name, _arguments) => (None, Some(floria::Call::new(name.clone().into(), Default::default()))),
        };

        let validator = match &attribute_definition.validation {
            Some(validation) => match validation {
                Self::Literal(_) => None, // TODO?
                Self::Call(name, _arguments) => Some(floria::Call::new(name.clone().into(), Default::default())),
            },
            None => None,
        };

        let mut floria_property = floria::Property::new(variant, updater, validator, false);
        floria_property.metadata.set_tosca_entity("Attribute");
        floria_property.metadata.set_tosca_description(attribute_definition.description.as_ref());
        floria_property.metadata.merge_tosca_metadata(&attribute_definition.metadata);

        catalog.data_types.add_floria_group_ids(
            &mut floria_property.group_ids,
            &"data".into(),
            index.index.get(&attribute_definition.type_name).unwrap(),
        );

        Ok(floria_property)
    }
}

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        if let Some((key, value)) = self.to_key_value_pair() {
            if let Variant::Text(key_text) = key {
                if key_text.inner.starts_with(FUNCTION_PREFIX) {
                    let key_string = &key_text.inner[1..];

                    // Escaped?
                    if key_string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let map =
                            Variant::from([(Variant::from(key_string).with_annotations_from(key_text), value.clone())]);
                        return Ok(Some(ValueAssignment::Literal(map.into())));
                    }

                    let mut arguments = Vec::default();
                    for argument in value.iterator() {
                        let argument: Option<ValueAssignment<_>> = argument.resolve_with_errors(errors)?;
                        if let Some(argument) = argument {
                            arguments.push(argument);
                        }
                    }

                    return Ok(Some(ValueAssignment::Call(key_text.clone(), arguments)));
                }
            }
        }

        Ok(Some(ValueAssignment::Literal(self.clone())))
    }
}

impl<AnnotatedT> Default for ValueAssignment<AnnotatedT> {
    fn default() -> Self {
        Self::Literal(Default::default())
    }
}

// Used by ArtifactDefinition
impl<AnnotatedT> Subentity<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _parent: &Self,
        _depot: &mut Depot,
        _source_id: &SourceID,
        _scope: &Scope,
        _errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        Ok(())
    }
}

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &PropertyDefinition<AnnotatedT>,
        _depot: &mut Depot,
        _source_id: &SourceID,
        _scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if self.is_undefined() {
            match &parent.default {
                Some(default) => *self = default.clone(),

                None => {
                    if parent.required {
                        errors.to_error_recipient().give(MissingRequiredError::new("property".into(), "?".into()))?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &AttributeDefinition<AnnotatedT>,
        _depot: &mut Depot,
        _source_id: &SourceID,
        _scope: &Scope,
        _errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if self.is_undefined()
            && let Some(default) = &parent.default
        {
            *self = default.clone();
        }

        Ok(())
    }
}

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &ParameterDefinition<AnnotatedT>,
        _depot: &mut Depot,
        _source_id: &SourceID,
        _scope: &Scope,
        _errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if self.is_undefined()
            && let Some(default) = &parent.default
        {
            *self = default.clone();
        }

        Ok(())
    }
}

// For ArtifactAssignment and ArtifactDefinition
impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, _scope: &Scope) -> Self {
        self.clone()
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, _scope: &Scope) -> ValueAssignment<AnnotatedT> {
        self.default.clone().unwrap_or_default()
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, _scope: &Scope) -> ValueAssignment<AnnotatedT> {
        self.default.clone().unwrap_or_default()
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, _scope: &Scope) -> ValueAssignment<AnnotatedT> {
        self.default.clone().unwrap_or_default()
    }
}

// Delegated

impl<AnnotatedT> Annotated for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations(),
            Self::Call(name, _arguments) => name.annotated.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations_mut(),
            Self::Call(name, _arguments) => name.annotated.get_annotations_mut(),
        }
    }
}

impl<AnnotatedT> fmt::Display for ValueAssignment<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => fmt::Display::fmt(literal, formatter),

            Self::Call(name, arguments) => {
                write!(formatter, "{}(", name)?;

                for (argument, last) in IterateWithLast::new(arguments) {
                    fmt::Display::fmt(argument, formatter)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, ")")
            }
        }
    }
}

impl<AnnotatedT> Debuggable for ValueAssignment<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(literal) => literal.write_debug_for(writer, context),

            Self::Call(name, arguments) => {
                context.separate(writer)?;
                context.theme.write_name(writer, name)?;
                context.theme.write_delimiter(writer, "(")?;

                let child_context = &context.child().with_format(DebugFormat::Compact).with_separator(false);
                for (argument, last) in IterateWithLast::new(arguments) {
                    argument.write_debug_for(writer, child_context)?;
                    if !last {
                        context.theme.write_delimiter(writer, ",")?;
                    }
                }

                context.theme.write_delimiter(writer, ")")
            }
        }
    }
}

// Conversions

impl<AnnotatedT> Into<floria::Expression> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Expression {
        match self {
            Self::Literal(literal) => floria::Expression::Literal(literal.clone().into_annotated()),
            Self::Call(name, arguments) => {
                let arguments: Vec<_> = arguments.into_iter().map(|argument| argument.into()).collect();
                floria::Call::new(name.clone().into(), arguments).into()
            }
        }
    }
}
