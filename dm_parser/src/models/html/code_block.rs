/// Modifies the HTML based on the configuration of `options.
/// escapeCodeTagInterpolation` and the fenced code blocks language
/// (if language starts `!` then options configuration is reversed).
///
/// Because we are looking at the fenced language, we'll also add that to
/// the payload being passed through as this could be valuable for _search_
/// or other meta features.
pub fn escape_code_interpolation() -> Self {
    todo!()
}
