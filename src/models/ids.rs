use std::fmt;
use std::fmt::Display;
use std::ops::AddAssign;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Ids {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trakt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvrage: Option<u64>,
}

impl Default for Ids {
    fn default() -> Self {
        Self {
            trakt: None,
            slug: None,
            tvdb: None,
            imdb: None,
            tmdb: None,
            tvrage: None,
        }
    }
}

impl AddAssign<Ids> for &mut Ids {
    fn add_assign(&mut self, rhs: Ids) {
        self.trakt = self.trakt.or(rhs.trakt);
        self.slug = self.slug.clone().or(rhs.slug);
        self.tvdb = self.tvdb.or(rhs.tvdb);
        self.imdb = self.imdb.clone().or(rhs.imdb);
        self.tmdb = self.tmdb.or(rhs.tmdb);
        self.tvrage = self.tvrage.or(rhs.tvrage);
    }
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
