/// Entity must support both into_expression and to_depict
macro_rules! output {
    ( $self:ident, $store:ident, $entity:ident $(,)? ) => {
        match $self.get_output_format() {
            Some(output_format) => {
                let expression = $entity.into_expression(true, &$store)?;
                let variant: Variant<WithoutAnnotations> = expression.into();
                compris::ser::Serializer::new(output_format)
                    .with_pretty(!$self.output_plain)
                    .with_base64($self.output_base64)
                    .print(&variant)
                    .expect("print");
            }

            None => {
                $entity.to_depict(&$store).print_default_depiction();
            }
        }
    };
}

pub(crate) use output;
