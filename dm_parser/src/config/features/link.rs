use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkOptions {
    root_dir: Option<String>,
    external_link_class: Option<String>,
    internal_link_class: Option<String>,
    relative_link_class: Option<String>,
    fully_qualified_link_class: Option<String>,
    anchor_tag_class: Option<String>,
    router_link_class: Option<String>,
    insecure_class: Option<String>,
    file_class: Option<String>,
    mailto_class: Option<String>,
    image_class: Option<String>,
    document_class: Option<String>,
    // TODO: fix this type
    rule_based_classes: Option<String>,
    external_target: Option<String>,
    external_rel: Option<String>,
    internal_target: Option<String>,
    internal_rel: Option<String>,
    use_router_links: Option<bool>,
    clean_index_routes: Option<bool>,
    clean_all_routes: Option<bool>,
    // post_processing: Option<>
}

impl LinkOptions {
    pub fn new() -> Self {
        LinkOptions {
            root_dir: None,
            external_link_class: None,
            internal_link_class: None,
            relative_link_class: None,
            fully_qualified_link_class: None,
            router_link_class: None,
            anchor_tag_class: None,
            insecure_class: None,
            file_class: None,
            mailto_class: None,
            image_class: None,
            document_class: None,
            rule_based_classes: None,
            external_target: None,
            external_rel: None,
            internal_target: None,
            internal_rel: None,
            use_router_links: None,
            clean_index_routes: None,
            clean_all_routes: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkConfig {
    root_dir: String,
    external_link_class: String,
    internal_link_class: String,
    relative_link_class: String,
    fully_qualified_link_class: String,
    anchor_tag_class: String,
    router_link_class: String,
    insecure_class: String,
    file_class: String,
    mailto_class: String,
    image_class: String,
    document_class: String,
    // TODO: fix this type
    rule_based_classes: String,
    external_target: String,
    external_rel: String,
    internal_target: Option<String>,
    internal_rel: Option<String>,
    use_router_links: bool,
    clean_index_routes: bool,
    clean_all_routes: bool,
    // post_processing: Option<>
}
impl Default for LinkConfig {
    fn default() -> Self {
        LinkConfig {
            root_dir: String::from("src/pages"),
            external_link_class: String::from("external-link"),
            internal_link_class: String::from("internal-link"),
            relative_link_class: String::from("relative-link"),
            fully_qualified_link_class: String::from("fq-link"),
            router_link_class: String::from("router-link"),
            anchor_tag_class: String::from("anchor-tag"),
            insecure_class: String::from("insecure"),
            file_class: String::from("file-link"),
            mailto_class: String::from("mailto-link"),
            image_class: String::from("image-reference"),
            document_class: String::from("doc-reference"),
            rule_based_classes: String::from(""),
            external_target: String::from("_blank"),
            external_rel: String::from("noreferrer noopener"),
            internal_target: None,
            internal_rel: None,
            use_router_links: true,
            clean_index_routes: true,
            clean_all_routes: true,
        }
    }
}

impl LinkConfig {
    pub fn with_options(options: LinkOptions) -> Self {
        let mut link = LinkConfig::default();

        if let Some(root_dir) = options.root_dir {
            link.root_dir = root_dir;
        }
        if let Some(external_link_class) = options.external_link_class {
            link.external_link_class = external_link_class;
        }
        if let Some(internal_link_class) = options.internal_link_class {
            link.internal_link_class = internal_link_class;
        }
        if let Some(relative_link_class) = options.relative_link_class {
            link.relative_link_class = relative_link_class;
        }
        if let Some(fully_qualified_link_class) = options.fully_qualified_link_class {
            link.fully_qualified_link_class = fully_qualified_link_class;
        }
        if let Some(router_link_class) = options.router_link_class {
            link.router_link_class = router_link_class;
        }
        if let Some(anchor_tag_class) = options.anchor_tag_class {
            link.anchor_tag_class = anchor_tag_class;
        }
        if let Some(insecure_class) = options.insecure_class {
            link.insecure_class = insecure_class;
        }
        if let Some(file_class) = options.file_class {
            link.file_class = file_class;
        }
        if let Some(mailto_class) = options.mailto_class {
            link.mailto_class = mailto_class;
        }
        if let Some(image_class) = options.image_class {
            link.image_class = image_class;
        }
        if let Some(document_class) = options.document_class {
            link.document_class = document_class;
        }
        if let Some(rule_based_classes) = options.rule_based_classes {
            link.rule_based_classes = rule_based_classes;
        }
        if let Some(external_target) = options.external_target {
            link.external_target = external_target;
        }
        if let Some(external_rel) = options.external_rel {
            link.external_rel = external_rel;
        }
        link.internal_target = options.internal_target;
        link.internal_rel = options.internal_rel;

        if let Some(use_router_links) = options.use_router_links {
            link.use_router_links = use_router_links;
        }
        if let Some(clean_index_routes) = options.clean_index_routes {
            link.clean_index_routes = clean_index_routes;
        }
        if let Some(clean_all_routes) = options.clean_all_routes {
            link.clean_all_routes = clean_all_routes;
        }

        link
    }
}
