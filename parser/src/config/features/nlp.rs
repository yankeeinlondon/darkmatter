use lingua::Language;
use serde::{Deserialize, Serialize};

pub struct LangDetectConfig {
    languages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UseLangDetection {
    DoNotUse,
    /// look for ALL known languages
    All,
    /// in the US the predominate languages are English and Spanish
    USA,
    /// Includes the languages:
    /// English, French, German, Spanish, Dutch, Belgium, Swiss, Norse,
    /// Swedish, Italian, Greek, Portuguese, and Danish
    Western,
    /// Includes the languages:
    /// Hindi, Arabic, Mandarin, Japanese, Indonesian, Vietnamese, Korean,
    /// Russian, and English
    Eastern,
    /// Most spoken languages which are:
    /// English, French, Spanish, Chinese, Hindi, Bengali, Arabic,
    /// Russian, Portuguese, and Indonesian
    Big10,
    /// Specify the languages you wish to look for
    Languages(Vec<Language>),
}

/// Options regarding what NLP algorithms to run and how to
/// configure them.
#[derive(Debug, Serialize, Deserialize)]
pub struct NlpOptions {
    /// Whether to have language detection run over the
    /// content of the Markdown. Performance will vary based
    /// on the number of languages which are to be evaluated.
    use_language_detection: Option<UseLangDetection>,
    /// If language detection is not employed _or_ you want a fallback language to use
    /// you may set this here.
    set_default_language: Option<Language>,
    generate_readability_score: Option<bool>,
    generate_read_time_estimate: Option<bool>,
    generate_sentiment_analysis: Option<bool>,
    /// Create word tokens from content.
    ///
    /// Stemming and stop-words will be used if the
    /// language is known (and supported).
    create_word_tokens: Option<bool>,
}

/// Finalized configuration for NLP options
#[derive(Debug, Serialize, Deserialize)]
pub struct NlpConfig {
    /// Whether to have language detection run over the
    /// content of the Markdown. Performance will vary based
    /// on the number of languages which are to be evaluated.
    use_language_detection: UseLangDetection,
    /// If language detection is not employed _or_ you want a fallback language to use
    /// you may set this here.    
    set_default_language: Option<Language>,
    generate_readability_score: bool,
    generate_read_time_estimate: bool,
    generate_sentiment_analysis: bool,
    /// Create word tokens from content.
    ///
    /// Stemming and stop-words will be used if the
    /// language is known (and supported).
    create_word_tokens: bool,
}

impl NlpConfig {
    pub fn default() -> Self {
        NlpConfig {
            use_language_detection: UseLangDetection::DoNotUse,
            set_default_language: Some(Language::English),
            generate_read_time_estimate: true,
            generate_readability_score: true,
            generate_sentiment_analysis: false,
            create_word_tokens: false,
        }
    }

    /// uses the _default_ settings as a baseline and then adjusts based on options passed in
    pub fn with_options(options: NlpOptions) -> Self {
        let config = NlpConfig::default();

        if let Some(use_language_detection) = options.use_language_detection {
            config.use_language_detection = use_language_detection;
        }
        if let Some(set_default_language) = options.set_default_language {
            config.set_default_language = Some(set_default_language);
        }
        if let Some(generate_read_time_estimate) = options.generate_read_time_estimate {
            config.generate_read_time_estimate = generate_read_time_estimate;
        }
        if let Some(generate_readability_score) = options.generate_readability_score {
            config.generate_readability_score = generate_readability_score;
        }
        if let Some(generate_sentiment_analysis) = options.generate_sentiment_analysis {
            config.generate_sentiment_analysis = generate_sentiment_analysis;
        }
        if let Some(create_word_tokens) = options.create_word_tokens {
            config.create_word_tokens = create_word_tokens;
        }

        config
    }
}
