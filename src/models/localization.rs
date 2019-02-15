#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub title: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Language {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Country {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Translation {
    pub title: String,
    pub overview: String,
    pub tagline: Option<String>,
    pub language: String,
}
