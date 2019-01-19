use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ids {
    pub trakt: Option<u64>,
    pub slug: Option<String>,
    pub tvdb: Option<u64>,
    pub imdb: Option<String>,
    pub tmdb: Option<u64>,
    pub tvrage: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdType {
    Trakt,
    IMDb,
    TMDb,
    TVDB,
}

impl Display for IdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            IdType::Trakt => "trakt",
            IdType::IMDb => "imdb",
            IdType::TMDb => "tmdb",
            IdType::TVDB => "tvdb",
        })
    }
}
