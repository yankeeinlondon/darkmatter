use super::markdown::{MarkdownContent, MarkdownContentRaw};
use crate::config::features::frontmatter::{ExcerptStrategy, FrontmatterEngineType};
use crate::config::Config;
use crate::errors::fm_err::FrontmatterError;
use gray_matter::engine::{JSON, TOML, YAML};
use gray_matter::ParsedEntity;
use gray_matter::{Matter, Pod};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use tracing::{debug, info, instrument};

const NOT_ALLOWED_PROPS: [&str; 3] = ["hash_extracted", "hash_defaults_applied", "hash_overrides"];
const KNOWN_PROPS: [&str; 12] = [
    "title",
    "description",
    "subject",
    "category",
    "name",
    "excerpt",
    "image",
    "image_height",
    "image_width",
    "layout",
    "requires_auth",
    "meta",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct FmHashValues {
    /// A hash value representing the frontmatter immediately after it is
    /// extracted from the `MarkdownContentRaw`
    pub extracted: Option<u64>,
    /// A hash value representing the frontmatter after the default values
    /// hook has been applied to the page content
    pub defaults_applied: Option<u64>,
    /// A hash value representing the frontmatter after both the _defaults_
    /// and _overrides_ hooks have been applied to the page content.    
    pub overrides_applied: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct MetaProperty {
    /// All `<meta>` tags are effectively just a key/value pairing
    /// which this property captures.
    ///
    /// Note that the "key" property will be different for various
    /// platforms so for each key we will populate the following
    /// properties when calling `create_meta_tag()`:
    /// - key
    /// - itemprop - _used by google searches_
    /// - name - _used by Twitter_
    /// - property - _used by Facebook and OpenGraph APIs_
    key: String,
    /// The "value" property will be exported as the `content` property
    /// of any meta tags
    value: Value,
    other: Option<HashMap<String, Value>>,
}

impl MetaProperty {
    pub fn new(key: &str, value: Value, other: Option<HashMap<String, Value>>) -> Self {
        MetaProperty {
            key: key.to_string(),
            value,
            other,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct GenericFrontmatter(HashMap<String, Value>);
impl Display for GenericFrontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())
    }
}
impl From<&Frontmatter> for GenericFrontmatter {
    fn from(fm: &Frontmatter) -> GenericFrontmatter {
        GenericFrontmatter::new(fm)
    }
}

impl GenericFrontmatter {
    pub fn new(fm: &Frontmatter) -> Self {
        GenericFrontmatter(serde_json::from_str(&fm.to_string()).unwrap())
    }

    pub fn get(&self, property: &str) -> Option<&Value> {
        self.0.get(property)
    }

    pub fn insert(&mut self, property: &str, value: &Value) {
        self.0.insert(property.to_string(), value.clone());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    default
)]
pub struct Frontmatter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_auth: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meta: Vec<MetaProperty>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
    /// A hash value representing the frontmatter immediately after it is
    /// extracted from the `MarkdownContentRaw` (or from a
    /// `Frontmatter::new()`, etc.)
    #[serde(skip_serializing)]
    hash_extracted: Option<u64>,
    /// A hash value representing the frontmatter after the default values
    /// hook has been applied to the page content
    #[serde(skip_serializing)]
    hash_defaults_applied: Option<u64>,
    /// A hash value representing the frontmatter after both the _defaults_
    /// and _overrides_ hooks have been applied to the page content.
    #[serde(skip_serializing)]
    hash_overrides_applied: Option<u64>,
}

impl Default for Frontmatter {
    #[instrument]
    fn default() -> Frontmatter {
        Frontmatter::new(None).unwrap()
    }
}

impl Display for Frontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        write!(f, "{}", json)
    }
}

impl From<GenericFrontmatter> for Frontmatter {
    fn from(fm: GenericFrontmatter) -> Self {
        debug!("generic fm:\n{:?}", &fm);
        Frontmatter::try_from(json!(fm)).unwrap()
    }
}

impl Into<String> for Frontmatter {
    #[instrument]
    fn into(self) -> String {
        json!(self).to_string()
    }
}

impl TryFrom<String> for Frontmatter {
    type Error = FrontmatterError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Frontmatter::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Frontmatter {
    type Error = FrontmatterError;

    fn try_from(value: &str) -> Result<Self, FrontmatterError> {
        Frontmatter::new(Some(serde_json::from_str(value)?))
    }
}

impl TryFrom<&Value> for Frontmatter {
    type Error = FrontmatterError;

    #[instrument]
    fn try_from(json: &Value) -> Result<Frontmatter, FrontmatterError> {
        let val = json.clone();
        Frontmatter::try_from(val)
    }
}

impl TryFrom<Value> for Frontmatter {
    type Error = FrontmatterError;

    #[instrument]
    fn try_from(json: Value) -> Result<Frontmatter, FrontmatterError> {
        let json = serde_json::to_string(&json)?;
        Frontmatter::try_from(json.as_str())
    }
}

impl TryFrom<&Pod> for Frontmatter {
    type Error = FrontmatterError;

    #[instrument]
    fn try_from(pod: &Pod) -> Result<Frontmatter, FrontmatterError> {
        let pod = pod.clone();

        Ok(Frontmatter::try_from(pod)?)
    }
}

impl TryFrom<Pod> for Frontmatter {
    type Error = FrontmatterError;

    #[instrument]
    fn try_from(pod: Pod) -> Result<Frontmatter, FrontmatterError> {
        debug!("Trying to deserialize a Pod from gray_matter to a JSON Value");
        let json: Value = pod.deserialize()?;
        info!("Used gray_matter to convert to JSON");

        Frontmatter::try_from(json)
    }
}

impl Frontmatter {
    #[instrument]
    pub fn new(json: Option<Value>) -> Result<Self, FrontmatterError> {
        if let Some(json) = json {
            let mut fm: Frontmatter = serde_json::from_value(json.clone())?;
            let hash = Some(fm.current_hash_value());
            fm.hash_extracted = hash;

            debug!(
                "New Frontmatter from JSON:\n{}\n\nis translated to {:?}",
                &json, &fm
            );

            Ok(fm)
        } else {
            Ok(Frontmatter {
                title: None,
                description: None,
                subject: None,
                category: None,
                name: None,
                excerpt: None,
                image: None,
                image_height: None,
                image_width: None,
                layout: None,
                requires_auth: None,
                meta: vec![],
                other: HashMap::new(),
                hash_extracted: None,
                hash_defaults_applied: None,
                hash_overrides_applied: None,
            })
        }
    }

    /// Receives _raw markdown_ content and returns the Frontmatter
    /// and the markdown content with the Frontmatter extracted
    #[instrument]
    pub fn extract(
        raw_md: &MarkdownContentRaw,
        config: &Config,
    ) -> Result<(MarkdownContent, Frontmatter), FrontmatterError> {
        let fm = Frontmatter::default();
        let matter: ParsedEntity;

        match &config.features.frontmatter.engine {
            FrontmatterEngineType::YAML => {
                let mut parser = Matter::<YAML>::new();
                info!(matter_parser = "YAML");
                if let Some(delimiter) = &config.features.frontmatter.delimiter {
                    parser.delimiter = delimiter.clone();
                }
                matter = parser.parse(&raw_md.content());
            }
            FrontmatterEngineType::JSON => {
                let mut parser = Matter::<JSON>::new();
                info!(matter_parser = "JSON");
                if let Some(delimiter) = &config.features.frontmatter.delimiter {
                    parser.delimiter = delimiter.clone();
                }
                matter = parser.parse(&raw_md.content());
            }
            FrontmatterEngineType::TOML => {
                let mut parser = Matter::<TOML>::new();
                info!(matter_parser = "TOML");
                if let Some(delimiter) = &config.features.frontmatter.delimiter {
                    parser.delimiter = delimiter.clone();
                }
                matter = parser.parse(&raw_md.content());
            }
        };

        let mut frontmatter = if let Some(fm) = matter.data {
            Frontmatter::try_from(&fm)?
        } else {
            debug!("gray_matter did not find anything so using empty Frontmatter as default");
            Frontmatter::default()
        };

        // Excerpt content extracted from the body parse
        let excerpt = fm.excerpt;
        let markdown = MarkdownContent::new(raw_md, &matter.content);
        // Work with excerpt based on strategy
        let preferred = match config.features.frontmatter.excerpt_strategy {
            ExcerptStrategy::Auto => [&frontmatter.excerpt, &excerpt],
            ExcerptStrategy::Delimited(_) => [&excerpt, &None],
            ExcerptStrategy::Frontmatter => [&frontmatter.excerpt, &None],
            ExcerptStrategy::None => [&None, &None],
        }
        .into_iter()
        .flat_map(|i| i)
        .nth(0);

        frontmatter.excerpt = if let Some(excerpt) = preferred {
            Some(excerpt.clone())
        } else {
            None
        };

        let hash = frontmatter.current_hash_value();
        frontmatter.hash_extracted = Some(hash);

        info!(
            "frontmatter extract() completed successfully:\n{:?}\n{:?}",
            &frontmatter, &markdown
        );
        Ok((markdown, frontmatter))
    }

    /// Converts Frontmatter to a generic representation
    /// where certain set based operations are easier
    pub fn to_generic(&self) -> GenericFrontmatter {
        GenericFrontmatter::new(self)
    }

    /// get the current hash value for this frontmatter
    pub fn current_hash_value(&self) -> u64 {
        dm_utils::hash(&self.to_string(), None)
    }

    /// Allows using a `GenericFrontmatter` to set values
    /// on this instance of `Frontmatter`
    fn set_values(&mut self, values: GenericFrontmatter) -> Result<(), FrontmatterError> {
        for (k, v) in values.0 {
            if NOT_ALLOWED_PROPS.contains(&k.as_str()) {
                return Err(FrontmatterError::PropertyCanNotBeSet(k, v));
            }
            if !KNOWN_PROPS.contains(&k.as_str()) {
                self.other.insert(k, v);
            } else {
                println!("{} is a known prop", &k);
                match k.as_str() {
                    "title" => {
                        self.title = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("title"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "description" => {
                        self.description = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("description"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "subject" => {
                        self.subject = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("subject"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "category" => {
                        self.category = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("category"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "name" => {
                        self.name = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("name"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "excerpt" => {
                        self.excerpt = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("excerpt"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "image" => {
                        self.image = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("image"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "image_height" => {
                        self.image_height = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("image_height"),
                                String::from("u32"),
                            )
                        })?);
                    }
                    "image_width" => {
                        debug!("Image width picked up as known prop");
                        self.image_width = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("image_width"),
                                String::from("u32"),
                            )
                        })?);
                    }
                    "layout" => {
                        self.layout = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("layout"),
                                String::from("String"),
                            )
                        })?);
                    }
                    "requires_auth" => {
                        self.requires_auth = Some(serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("requires_auth"),
                                String::from("bool"),
                            )
                        })?);
                    }
                    "meta" => {
                        self.meta = serde_json::from_value(v).map_err(|_| {
                            FrontmatterError::PropertyIsWrongType(
                                String::from("meta"),
                                String::from("Vec<MetaProperty>"),
                            )
                        })?;
                    }
                    &_ => (),
                }
            }
        }

        Ok(())
    }

    /// Receive all the hash values which this Frontmatter instance
    /// has accumulated.
    pub fn hash_values(&self) -> FmHashValues {
        FmHashValues {
            extracted: self.hash_extracted,
            defaults_applied: self.hash_defaults_applied,
            overrides_applied: self.hash_overrides_applied,
        }
    }

    pub fn apply_default_values(
        &mut self,
        default_values: &Frontmatter,
    ) -> Result<(), FrontmatterError> {
        let selfy = self.clone();
        let mut current = GenericFrontmatter::from(&selfy);
        let default_values = GenericFrontmatter::from(default_values);

        for (k, v) in default_values.0 {
            if current.get(&k).is_none() {
                current.insert(&k, &v);
            }
        }

        self.set_values(current)?;
        self.hash_defaults_applied = Some(self.current_hash_value());

        Ok(())
    }

    pub fn apply_override_values(
        &mut self,
        overrides: &Frontmatter,
    ) -> Result<(), FrontmatterError> {
        let selfy = self.clone();
        let mut current = GenericFrontmatter::from(&selfy);
        let overrides = GenericFrontmatter::from(overrides);

        for (k, v) in overrides.0 {
            current.insert(&k, &v);
        }

        self.set_values(current)?;
        self.hash_overrides_applied = Some(self.current_hash_value());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tracing::{info, Level};
    use tracing_subscriber;

    const SIMPLE_MD: &str = r#"---
title: testing
foo: "bar"
bar: true
baz: 42
---
# Hello World\n
this is a test 
"#;

    fn trace(lvl: Level) {
        let collector = tracing_subscriber::fmt()
            // filter spans/events with level TRACE or higher.
            .with_max_level(lvl)
            .pretty()
            // build but do not install the subscriber.
            .finish();

        tracing::subscriber::set_global_default(collector).unwrap();

        info!("Tracing for tests enabled");
    }

    #[test]
    fn fm_extract_gives_valid_fm() {
        let md = MarkdownContentRaw::new(SIMPLE_MD);

        let (_, fm) = Frontmatter::extract(&md, &Config::default()).unwrap();

        let foo = fm.other.get("foo");
        let bar = fm.other.get("bar");
        let baz = fm.other.get("baz");

        assert!(fm.title.is_some());
        assert!(foo.is_some());
        assert!(bar.is_some());
        assert!(baz.is_some());

        if let Some(title) = fm.title {
            assert_eq!(title, "testing");
        };
        if let Some(foo) = foo {
            assert_eq!(foo, &Value::String("bar".to_string()))
        }
        if let Some(bar) = bar {
            let value = json!(true);
            assert_eq!(bar, &value);
        }
        if let Some(baz) = baz {
            let value = json!(42);
            assert_eq!(baz, &value);
        }
    }

    #[test]
    fn fm_extract_gives_valid_md() {
        let md = MarkdownContentRaw::new(SIMPLE_MD);
        let (md, _) = Frontmatter::extract(&md, &Config::default()).unwrap();

        assert_eq!(md.content().contains("foo: \"bar\""), false)
    }

    #[test]
    fn fm_from_json() {
        let json = json!({"color":"red","imageWidth":500,"title":"test"});
        let fm = Frontmatter::try_from(json).unwrap();

        assert_eq!(fm.title, Some("test".to_string()));
        assert_eq!(fm.image_width, Some(500));
    }

    /// Ensure that you can move into a generic and then back again
    /// with no value loss (although hashes beyond the first will
    /// be lost).
    #[test]
    fn fm_generic_identity() {
        trace(Level::WARN);
        let json = json!({
            "title": "test",
            "color": "red",
            "imageWidth": 500
        });
        println!("{:?}", &json);
        let fm = Frontmatter::try_from(json).unwrap();

        let initial_hash = fm.hash_extracted.clone();
        assert!(initial_hash.is_some());

        // convert to generic
        let generic = fm.to_generic();

        // values transferred
        assert_eq!(
            generic.get("title"),
            Some(&Value::String("test".to_string()))
        );

        // only extracted hash available in identity
        assert!(generic.get("hash_extracted").is_none());

        // back to Frontmatter
        let identity = Frontmatter::from(generic);

        assert_eq!(&identity.title, &Some("test".to_string()));
        assert_eq!(&identity.image_width, &Some(500));
        assert_eq!(
            &identity.other.get("color"), //
            &Some(&Value::String("red".to_string()))
        );
    }

    #[test]
    fn fm_defaults_overrides_and_hashes() {
        let (_, mut fm) = MarkdownContentRaw::new(SIMPLE_MD)
            .parse(&Config::default())
            .unwrap();
        let hashes = fm.hash_values();

        assert!(hashes.extracted.is_some());
        assert!(hashes.defaults_applied.is_none());
        assert!(hashes.overrides_applied.is_none());
        let dv = Frontmatter::try_from(json!({ "hokey": "pokey", "bar": false })).unwrap();

        fm.apply_default_values(&dv).unwrap();
        let hashes = fm.hash_values();

        assert!(hashes.extracted.is_some());
        assert!(hashes.defaults_applied.is_some());
        assert!(hashes.overrides_applied.is_none());

        // default of "false" is given up when value overrides
        assert_eq!(fm.other.get("bar"), Some(&Value::Bool(true)));
        // "hokey" is not defined on the JSON so the default value is used
        assert_eq!(
            fm.other.get("hokey"),
            Some(&Value::String("pokey".to_string()))
        );
        // the "title" provided in the JSON is retained after defaults applied
        assert_eq!(fm.title, Some("testing".to_string()));

        let dv = Frontmatter::try_from(json!({
            "action": "turn yourself around",
            "hokey": "smokey",
            "title": "who's testing now?"
        }))
        .unwrap();

        fm.apply_override_values(&dv).unwrap();
        let hashes = fm.hash_values();

        // even though the JSON had a title; the override takes precedent
        assert_eq!(fm.title, Some("who's testing now?".to_string()));
        // overrides also set properties which weren't found on the JSON
        assert_eq!(
            fm.other.get("action"),
            Some(&Value::String("turn yourself around".to_string()))
        );
        // same applies for properties set as a default value
        assert_eq!(
            fm.other.get("hokey"),
            Some(&Value::String("smokey".to_string()))
        );

        assert!(hashes.extracted.is_some());
        assert!(hashes.defaults_applied.is_some());
        assert!(hashes.overrides_applied.is_some());
    }
}
