use crate::models::{Episode, Movie, Show, WatchableType};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Watching {
    expires_at: DateTime<Utc>,
    started_at: DateTime<Utc>,
    action: String,
    #[serde(rename = "type")]
    item_type: WatchableType,
    episode: Option<Episode>,
    show: Option<Show>,
    movie: Option<Movie>,
}
