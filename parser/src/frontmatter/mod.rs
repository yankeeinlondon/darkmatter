use gray_matter::engine::YAML;
use gray_matter::Matter;

use crate::config::frontmatter::Frontmatter;
use crate::config::Options;

/// Extracts Frontmatter from Markdown content and applies _default values_
/// as well as _overrides_.
pub fn extract_frontmatter(
    filename: &str,
    content: &str,
    options: &Options,
) -> (String, Frontmatter) {
    let matter = Matter::<YAML>::new();
    let fm = matter.parse(content);

    (fm.content, Frontmatter::from(fm.data))
}
