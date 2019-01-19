use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionShow {
    title: Option<String>,
    year: Option<u16>,
    ids: Option<Ids>,
}

impl OptionShow {
    pub fn new(trakt_slug: String) -> Self {
        Self {
            title: None,
            year: None,
            ids: Some(Ids {
                trakt: None,
                slug: Some(trakt_slug),
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            }),
        }
    }
}

impl From<Show> for OptionShow {
    fn from(show: Show) -> Self {
        Self {
            title: Some(show.title),
            year: show.year,
            ids: Some(show.ids),
        }
    }
}
