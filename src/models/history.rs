//! All models related to [histories]
//!
//! [histories]: https://trakt.docs.apiary.io/#reference/users/history
use crate::extended_info::{WithFull, WithNone};
use crate::models::{Episode, FullEpisode, FullMovie, FullShow, Movie, Show, WatchableType};
use chrono::{DateTime, Utc};

/// An item in an user's [history]
///
/// [history]: https://trakt.docs.apiary.io/#reference/users/history
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: u64,
    pub watched_at: DateTime<Utc>,
    pub action: String,
    #[serde(rename = "type")]
    pub item_type: WatchableType,
    pub movie: Option<Movie>,
    pub episode: Option<Episode>,
    pub show: Option<Show>,
}

impl WithFull for HistoryItem {
    type Full = FullHistoryItem;
}

/// An item in an user's [history] with full [extended info]
///
/// [history]: https://trakt.docs.apiary.io/#reference/users/history
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullHistoryItem {
    pub id: u64,
    pub watched_at: DateTime<Utc>,
    pub action: String,
    #[serde(rename = "type")]
    pub item_type: WatchableType,
    pub movie: Option<FullMovie>,
    pub episode: Option<FullEpisode>,
    pub show: Option<FullShow>,
}

impl WithNone for FullHistoryItem {
    type None = HistoryItem;
}
