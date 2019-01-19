use crate::models::{Episode, Movie, OptionEpisode, OptionMovie, OptionShow, Show};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkin {
    pub movie: Option<OptionMovie>,
    pub show: Option<OptionShow>,
    pub episode: Option<OptionEpisode>,
    pub sharing: CheckinSharing,
    pub message: String,
    pub app_version: String,
    pub app_date: NaiveDate,
}

impl Checkin {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinSharing {
    twitter: bool,
    tumblr: bool,
    facebook: bool,
}

impl CheckinSharing {
    pub fn new(twitter: bool, tumblr: bool, facebook: bool) -> CheckinSharing {
        CheckinSharing {
            twitter,
            tumblr,
            facebook,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinResponse {
    id: u64,
    watched_at: DateTime<Utc>,
    sharing: CheckinSharing,
    movie: Option<Movie>,
    episode: Option<Episode>,
    show: Option<Show>,
}