use crate::models::{Episode, Movie, Show, WatchableType};
use chrono::{DateTime, Utc};

/// The item an user is [currently watching]
///
/// [currently watching]: https://trakt.docs.apiary.io/#reference/users/watching/get-watching
#[derive(Debug, Serialize, Deserialize)]
pub struct Watching {
    pub expires_at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub action: String,
    #[serde(rename = "type")]
    pub item_type: WatchableType,
    pub episode: Option<Episode>,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}
