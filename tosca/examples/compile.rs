use {
    compris::annotate::*,
    floria::*,
    kutil::{cli::depict::*, std::error::*},
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    read_url::*,
};

pub fn main() {
    let source = b"
tosca_definitions_version: tosca_2_0

node_types:
    MyNode: {}

service_template:
    node_templates:
        node:
            type: MyNode
";

    // Add source to URL context as internal URL

    let url_context = UrlContext::new();
    url_context
        .register_internal_url("/hello_world.yaml".into(), true, None, Some("text".into()), source)
        .expect("register_internal_url");

    let source_id = SourceID::URL("internal:///hello_world.yaml".into());

    // Create catalog with TOSCA 2.0 dialect

    let mut catalog = Catalog::default();
    catalog.add_dialect_ref(tosca_2_0::Dialect::default().into());
    catalog.add_source(tosca_2_0::Dialect::implicit_source::<WithAnnotations>()); // TODO: without?

    // Load our source

    catalog
        .load_source_with_annotations::<WithAnnotations, _>(&source_id, &url_context, &mut FailFastErrorRecipient)
        .unwrap();

    // Complete

    catalog.complete_entities::<WithAnnotations, _>(&mut FailFastErrorRecipient).unwrap();

    // Compile service template into Floria in-memory store

    let store = InMemoryStore::default();

    let service_template_id = catalog
        .compile_service_template::<WithAnnotations, _>(
            &Directory::default(),
            store.to_ref(),
            &source_id,
            &mut FailFastErrorRecipient,
        )
        .unwrap()
        .expect("compile_service_template");

    // Print service template

    let service_template = store.get_vertex_template(&service_template_id).unwrap().expect("get_vertex_template");
    service_template.to_depict(&store).print_default_depiction();
}
