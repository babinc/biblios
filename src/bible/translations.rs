/// Available Bible translations that can be downloaded
///
/// These are translations that are either public domain or have
/// permissive licenses allowing free distribution.

#[derive(Debug, Clone)]
pub struct TranslationInfo {
    /// Short identifier (e.g., "KJV", "ASV")
    pub id: &'static str,
    /// Full name
    pub name: &'static str,
    /// Brief description
    pub description: &'static str,
    /// Language code
    pub language: &'static str,
    /// URL to download the JSON data
    pub url: &'static str,
    /// Whether this is the default/recommended translation
    pub is_default: bool,
}

/// All available translations
pub const AVAILABLE_TRANSLATIONS: &[TranslationInfo] = &[
    TranslationInfo {
        id: "KJV",
        name: "King James Version",
        description: "Classic English translation (1611)",
        language: "en",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/en_kjv.json",
        is_default: true,
    },
    TranslationInfo {
        id: "BBE",
        name: "Bible in Basic English",
        description: "Simple vocabulary, easy to read",
        language: "en",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/en_bbe.json",
        is_default: false,
    },
    TranslationInfo {
        id: "ASV",
        name: "American Standard Version",
        description: "Literal translation (1901)",
        language: "en",
        url: "https://raw.githubusercontent.com/scrollmapper/bible_databases/master/json/t_asv.json",
        is_default: false,
    },
    TranslationInfo {
        id: "WEB",
        name: "World English Bible",
        description: "Modern public domain translation",
        language: "en",
        url: "https://raw.githubusercontent.com/scrollmapper/bible_databases/master/json/t_web.json",
        is_default: false,
    },
    TranslationInfo {
        id: "YLT",
        name: "Young's Literal Translation",
        description: "Highly literal translation (1862)",
        language: "en",
        url: "https://raw.githubusercontent.com/scrollmapper/bible_databases/master/json/t_ylt.json",
        is_default: false,
    },
    // Spanish
    TranslationInfo {
        id: "RVR",
        name: "Reina-Valera",
        description: "Spanish classic translation",
        language: "es",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/es_rvr.json",
        is_default: false,
    },
    // Portuguese
    TranslationInfo {
        id: "AA",
        name: "Almeida Atualizada",
        description: "Portuguese translation",
        language: "pt",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/pt_aa.json",
        is_default: false,
    },
    // German
    TranslationInfo {
        id: "SCH",
        name: "Schlachter",
        description: "German translation",
        language: "de",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/de_schlachter.json",
        is_default: false,
    },
    // French
    TranslationInfo {
        id: "FRA",
        name: "French APEE",
        description: "French translation",
        language: "fr",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/fr_apee.json",
        is_default: false,
    },
    // Chinese
    TranslationInfo {
        id: "CUV",
        name: "Chinese Union Version",
        description: "Traditional Chinese",
        language: "zh",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/zh_cuv.json",
        is_default: false,
    },
    // Russian
    TranslationInfo {
        id: "SYN",
        name: "Synodal Translation",
        description: "Russian Orthodox translation",
        language: "ru",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/ru_synodal.json",
        is_default: false,
    },
    // Korean
    TranslationInfo {
        id: "KOR",
        name: "Korean Bible",
        description: "Korean translation",
        language: "ko",
        url: "https://raw.githubusercontent.com/thiagobodruk/bible/master/json/ko_ko.json",
        is_default: false,
    },
];

/// Get a translation by ID
pub fn get_translation(id: &str) -> Option<&'static TranslationInfo> {
    AVAILABLE_TRANSLATIONS.iter().find(|t| t.id.eq_ignore_ascii_case(id))
}

/// Get all English translations
pub fn english_translations() -> Vec<&'static TranslationInfo> {
    AVAILABLE_TRANSLATIONS.iter().filter(|t| t.language == "en").collect()
}

/// Get the default translation
pub fn default_translation() -> &'static TranslationInfo {
    AVAILABLE_TRANSLATIONS.iter().find(|t| t.is_default).unwrap_or(&AVAILABLE_TRANSLATIONS[0])
}
