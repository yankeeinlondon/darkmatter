use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaOptions {
    /// The frontmatter property which will determine the `<title>`
    /// property in the HEAD.
    ///
    /// @default title
    pub title_prop: Option<String>,
    /// Properties in frontmatter dictionary which will be treated as "meta property"
    /// and converted into a `<meta>` tag in the header.
    ///
    /// Note: the `meta` property is always considered an array of meta properties
    pub meta_props: Option<Vec<String>>,
    pub route_meta_props: Option<Vec<String>>,

    // TODO: remember to add "routePath" as hook and have vite integration map; also consider if the same approach should be taken with "routeName"
    /// This defines the name of the _frontmatter property_ which will map to the
    /// route's "name". If this property is set in a page's frontmatter then the page
    ///  will import Vue Router to set the name.
    ///
    /// **Note:** if you need a _per-page_ way of changing this then use the
    /// `route_name` hook instead.
    pub route_name_prop: Option<String>,

    /// Allows query parameters on the page to be passed into Frontmatter properties
    ///
    /// @default false
    pub query_parameters: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaConfig {
    /// The frontmatter property which will determine the `<title>`
    /// property in the HEAD.
    pub title_prop: String,
    /// Properties in frontmatter dictionary which will be treated as "meta property"
    /// and converted into a `<meta>` tag in the header.
    ///
    /// Note: the `meta` property is always considered an array of meta properties
    pub meta_props: Vec<String>,
    pub route_meta_props: Vec<String>,

    // TODO: remember to add "routePath" as hook and have vite integration map; also consider if the same approach should be taken with "routeName"
    /// This defines the name of the _frontmatter property_ which will map to the
    /// route's "name". If this property is set in a page's frontmatter then the page
    ///  will import Vue Router to set the name.
    ///
    /// **Note:** if you need a _per-page_ way of changing this then use the
    /// `route_name` hook instead.
    pub route_name_prop: String,

    /// Allows query parameters on the page to be passed into Frontmatter properties
    ///
    /// @default false
    pub query_parameters: bool,
}

impl MetaConfig {
    pub fn default() -> Self {
        MetaConfig {
            title_prop: "title".to_string(),
            meta_props: vec![
                "title".to_string(),
                "description".to_string(),
                "image".to_string(),
                "url".to_string(),
                "image_width".to_string(),
                "image_height".to_string(),
            ],
            route_meta_props: vec!["layout".to_string()],
            route_name_prop: "routeName".to_string(),
            query_parameters: false,
        }
    }

    pub fn with_options(options: MetaOptions) -> Self {
        let mut config = MetaConfig::default();

        if let Some(meta_props) = options.meta_props {
            config.meta_props = meta_props;
        }
        if let Some(route_meta_props) = options.route_meta_props {
            config.route_meta_props = route_meta_props;
        }
        if let Some(route_name_prop) = options.route_name_prop {
            config.route_name_prop = route_name_prop;
        }
        if let Some(query_parameters) = options.query_parameters {
            config.query_parameters = query_parameters;
        }

        config
    }
}
