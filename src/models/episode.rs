use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub season: u32,
    pub number: u32,
    pub title: Option<String>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionEpisode {
    pub season: Option<u32>,
    pub number: Option<u32>,
    pub title: Option<String>,
    pub ids: Option<Ids>,
}

impl OptionEpisode {
    pub fn new(trakt_id: u64) -> Self {
        Self {
            season: None,
            number: None,
            title: None,
            ids: Some(Ids {
                trakt: Some(trakt_id),
                slug: None,
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            }),
        }
    }

    pub fn by_season(season_number: u32, episode_number: u32) -> Self {
        Self {
            season: Some(season_number),
            number: Some(episode_number),
            title: None,
            ids: None,
        }
    }
}

impl From<Episode> for OptionEpisode {
    fn from(ep: Episode) -> Self {
        Self {
            season: Some(ep.season),
            number: Some(ep.number),
            title: ep.title,
            ids: Some(ep.ids),
        }
    }
}
