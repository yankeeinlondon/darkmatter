use gray_matter::engine::YAML;
use gray_matter::Matter;

pub mod errors;
pub mod frontmatter;

use crate::config::Config;
use crate::models::fm::Frontmatter;
use crate::models::md::MarkdownContent;

/// This function separates Frontmatter from the Markdown and returns both
/// after applying all appropriate hook/event transforms on them.
///
/// Event Hooks:
/// - `fm_default_values`
/// - `md_raw_content`
pub fn extract_frontmatter(
    id: &str,
    content: &str,
    config: &Config,
) -> Result<(MarkdownContent, Frontmatter), Error> {
    let matter = Matter::<YAML>::new();
    let fm = matter.parse(content);
    let content = MarkdownContent::new(&fm.content);
    let fm = match fm.data {
        Some(data) => Frontmatter::from_matter(data),
        None => Frontmatter::new(),
    };

    Ok((content, fm))
}
