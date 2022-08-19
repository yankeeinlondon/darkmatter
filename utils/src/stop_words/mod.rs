use human_regex::{
    beginning_of_text, exactly, one_or_more, or, punctuation, whitespace, word_boundary,
};
use stop_words::{get, LANGUAGE};

pub fn extract_stop_words(text: &str, lang: LANGUAGE) -> String {
    let text = text.to_ascii_lowercase();
    let punk = one_or_more(punctuation());
    let text = punk.to_regex().replace_all(&*text, "");
    let words = get(lang);

    let stop =
        word_boundary() + exactly(1, or(&words)) + word_boundary() + one_or_more(whitespace());

    let text = stop.to_regex().replace_all(&*text, " ");
    let extra_whitespace = one_or_more(whitespace());
    let text = extra_whitespace.to_regex().replace_all(&*text, " ");
    let trimmings = beginning_of_text() + one_or_more(whitespace());
    let text = trimmings.to_regex().replace_all(&*text, "");

    text.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn english_words() {
        let prose = "The quick brown fox jumped over the lazy dog.";
        let stop = extract_stop_words(prose, LANGUAGE::English);

        assert_eq!(stop, String::from("quick brown fox jumped lazy dog"))
    }

    #[test]
    fn spanish_words() {
        let prose = "El veloz zorro marrón saltó sobre el perezoso perro.";
        let stop = extract_stop_words(prose, LANGUAGE::Spanish);

        assert_eq!(
            stop,
            String::from("veloz zorro marrón saltó perezoso perro")
        )
    }
}
