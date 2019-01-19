#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    name: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountryMedia {
    #[serde(rename = "movies")]
    Movies,
    #[serde(rename = "shows")]
    Shows,
}

impl ToString for CountryMedia {
    fn to_string(&self) -> String {
        String::from(
            match self {
                CountryMedia::Movies => "movies",
                CountryMedia::Shows => "shows",
            }
        )
    }
}