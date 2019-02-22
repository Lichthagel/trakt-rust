/// An [alias]
///
/// [alias]: https://trakt.docs.apiary.io/#reference/movies/aliases
#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub title: String,
    pub country: String,
}

/// A [language]
///
/// [language]: https://trakt.docs.apiary.io/#reference/languages
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Language {
    pub name: String,
    pub code: String,
}

/// A [country]
///
/// [country]: https://trakt.docs.apiary.io/#reference/countries
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Country {
    pub name: String,
    pub code: String,
}

/// A [translation]
///
/// [translation]: https://trakt.docs.apiary.io/#reference/movies/translations
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Translation {
    pub title: String,
    pub overview: String,
    pub tagline: Option<String>,
    pub language: String,
}
