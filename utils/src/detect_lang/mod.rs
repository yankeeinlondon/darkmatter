use std::cmp::Ordering;

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

#[derive(Debug, PartialEq)]
pub enum LanguageResult {
    /// Presents when language detection has found a language
    /// with a confidence score above the confidence threshold
    /// _and_ where no other languages are above it.
    Confident(Language),
    /// More than one language is above the confidence threshold
    /// then the high-confidence options will be listed along with
    /// the confidence level
    MultipleChoices(Vec<(Language, f64)>),
    /// Language detection _did_ find one or more possible languages
    /// but none met the confidence threshold
    Unsure(Vec<(Language, f64)>),
    /// Language detection was run but no known languages were detected
    NothingFound(),
    /// Language detection was not run
    NotChecked(),
}

pub enum LanguageScope {
    /// All languages
    All(),
    /// With "include" mode any languages expressed will be exclusively _included_ as possible
    /// language options whereas all others will be ignored.
    Include(&'static [Language]),
    /// With "exclude" mode the starting point will be ALL languages with the _exclusion_ of those
    /// stated as not possible.
    Exclude(&'static [Language]),
    /// Includes only languages which are written with a Latin script
    Latin(),
    /// Includes only languages which are written with a Cyrillic script
    Cyrillic(),
    /// Includes only languages which are written with a Devanagari script
    Devanagari(),
    /// Includes all spoken languages (effectively this means it excludes Latin)
    Spoken(),
}

pub struct LanguageOptions {
    /// The languages which will be considered when trying to identify a language within a corpus.
    /// - [available languages](https://docs.rs/lingua/latest/lingua/enum.Language.html)
    /// - use the LanguageScope enum to choose between include, exclude, cyrillic, and spoken
    pub languages: LanguageScope,
    /// An integer value between 1 and 100 which maps to
    /// percentage confidence value which the caller is willing
    /// to accept as "truthful".
    pub confidence_threshold: Option<f64>,

    /// Helps to address when the same word has the same spelling in more than
    /// one language. The default value is `0.00`.
    ///
    /// [Docs](https://github.com/pemistahl/lingua-rs#92-minimum-relative-distance)
    pub min_relative_distance: f64,

    /// By default language models are lazy loaded but this can be switched where
    /// appropriate.
    ///
    /// [Docs](https://github.com/pemistahl/lingua-rs#94-eager-loading-versus-lazy-loading)
    eager_loading: bool,
}

impl LanguageOptions {
    pub fn new(
        languages: LanguageScope,
        confidence_threshold: Option<f64>,
        min_relative_distance: f64,
        eager_loading: bool,
    ) -> Self {
        LanguageOptions {
            languages,
            confidence_threshold,
            min_relative_distance,
            eager_loading,
        }
    }

    pub fn default() -> Self {
        LanguageOptions {
            languages: LanguageScope::All(),
            confidence_threshold: Some(0.75),
            min_relative_distance: 0.00,
            eager_loading: false,
        }
    }

    pub fn all_with_confidence(confidence_threshold: Option<f64>) -> Self {
        let mut lang = LanguageOptions::default();
        lang.confidence_threshold = confidence_threshold;

        lang
    }

    pub fn whitelist(languages: &'static [Language]) -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Include(languages);

        lang
    }

    pub fn whitelist_with_threshold(
        languages: &'static [Language],
        confidence_threshold: Option<f64>,
    ) -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Include(languages);
        lang.confidence_threshold = confidence_threshold;

        lang
    }

    pub fn blacklist(languages: &'static [Language]) -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Exclude(languages);

        lang
    }

    pub fn blacklist_with_threshold(
        languages: &'static [Language],
        confidence_threshold: Option<f64>,
    ) -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Exclude(languages);
        lang.confidence_threshold = confidence_threshold;

        lang
    }

    pub fn spoken() -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Spoken();

        lang
    }

    pub fn latin() -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Latin();

        lang
    }

    pub fn cyrillic() -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Cyrillic();

        lang
    }

    pub fn devanagari() -> Self {
        let mut lang = LanguageOptions::default();
        lang.languages = LanguageScope::Devanagari();

        lang
    }

    pub fn load_eagerly(self) -> bool {
        self.eager_loading
    }
}

fn get_detector(options: &LanguageOptions) -> LanguageDetector {
    let mut detector = match options.languages {
        LanguageScope::All() => LanguageDetectorBuilder::from_all_languages(),
        LanguageScope::Latin() => LanguageDetectorBuilder::from_all_languages_with_latin_script(),
        LanguageScope::Cyrillic() => {
            LanguageDetectorBuilder::from_all_languages_with_cyrillic_script()
        }
        LanguageScope::Devanagari() => {
            LanguageDetectorBuilder::from_all_languages_with_devanagari_script()
        }
        LanguageScope::Spoken() => LanguageDetectorBuilder::from_all_spoken_languages(),
        LanguageScope::Include(languages) => LanguageDetectorBuilder::from_languages(languages),
        LanguageScope::Exclude(languages) => {
            LanguageDetectorBuilder::from_all_languages_without(languages)
        }
    };

    detector.with_minimum_relative_distance(options.min_relative_distance);

    if options.eager_loading {
        detector.with_preloaded_language_models();
    }

    let detector = detector.build();

    detector
}

/// given a corpus of text, this function will
/// use the [lingua-rs](https://github.com/pemistahl/lingua-rs)
/// crate to try to detect the underlying language.
pub fn detect_language(text: &str, options: LanguageOptions) -> LanguageResult {
    let detector = get_detector(&options);
    if let Some(threshold) = options.confidence_threshold {
        let distribution = detector.compute_language_confidence_values(text);
        let mut above: Vec<(Language, f64)> = vec![];
        let mut below: Vec<(Language, f64)> = vec![];
        for (lang, confidence) in distribution {
            if confidence >= threshold {
                above.push((lang, confidence));
            } else {
                below.push((lang, confidence));
            }
        }
        if above.len() == 1 {
            let (lang, _) = above.get(0).unwrap();
            return LanguageResult::Confident(lang.clone());
        } else if above.len() == 0 {
            if below.len() > 0 {
                return LanguageResult::Unsure(below);
            } else {
                return LanguageResult::NothingFound();
            }
        } else {
            // multiple languages in "above" so sort by confidence
            above.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            let s = &above[0..2].to_owned();
            let front: Vec<f64> = s.iter().map(|(l, c)| c.clone()).into_iter().collect();

            if front
                .get(0)
                .unwrap()
                .partial_cmp(front.get(1).unwrap())
                .unwrap()
                == Ordering::Equal
            {
                return LanguageResult::MultipleChoices(above);
            } else {
                let lang = above.get(0).unwrap().clone();
                return LanguageResult::Confident(lang.0);
            }
        }
    } else {
        let found = detector.detect_language_of(text);
        if let Some(found) = found {
            return LanguageResult::Confident(found);
        } else {
            return LanguageResult::NothingFound();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EN_TEXT: &str = "The quick brown fox jumped over the lazy dog";
    const FR_TEXT: &str = "Le rapide renard brun sauta par dessus le chien paresseux";
    const DE_TEXT: &str = "Der schnelle braune Fuchs sprang über den faulen Hund";
    const ES_TEXT: &str = "El rápido zorro marrón saltó sobre el perro perezoso";
    const IT_TEXT: &str = "La volpe marrone veloce saltò sul cane pigro";

    const LANG_SUBSET: [Language; 5] = [
        Language::English,
        Language::French,
        Language::German,
        Language::Spanish,
        Language::Italian,
    ];

    #[test]
    fn correct_with_limited_languages() {
        let en = detect_language(EN_TEXT, LanguageOptions::whitelist(&LANG_SUBSET));
        let fr = detect_language(FR_TEXT, LanguageOptions::whitelist(&LANG_SUBSET));
        let de = detect_language(DE_TEXT, LanguageOptions::whitelist(&LANG_SUBSET));
        let es = detect_language(ES_TEXT, LanguageOptions::whitelist(&LANG_SUBSET));
        let it = detect_language(IT_TEXT, LanguageOptions::whitelist(&LANG_SUBSET));

        assert_eq!(en, LanguageResult::Confident(Language::English));
        assert_eq!(fr, LanguageResult::Confident(Language::French));
        assert_eq!(de, LanguageResult::Confident(Language::German));
        assert_eq!(es, LanguageResult::Confident(Language::Spanish));
        assert_eq!(it, LanguageResult::Confident(Language::Italian));
    }

    #[test]
    fn correct_with_no_threshold() {
        let en = detect_language(EN_TEXT, LanguageOptions::all_with_confidence(None));
        let fr = detect_language(FR_TEXT, LanguageOptions::all_with_confidence(None));
        let de = detect_language(DE_TEXT, LanguageOptions::all_with_confidence(None));
        let es = detect_language(ES_TEXT, LanguageOptions::all_with_confidence(None));
        let it = detect_language(IT_TEXT, LanguageOptions::all_with_confidence(None));

        assert_eq!(en, LanguageResult::Confident(Language::English));
        assert_eq!(fr, LanguageResult::Confident(Language::French));
        assert_eq!(de, LanguageResult::Confident(Language::German));
        assert_eq!(es, LanguageResult::Confident(Language::Spanish));
        assert_eq!(it, LanguageResult::Confident(Language::Italian));
    }
}
