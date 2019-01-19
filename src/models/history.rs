use crate::models::{Episode, Movie, WatchableType};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItem {
    id: u64,
    watched_at: DateTime<Utc>,
    action: String,
    #[serde(rename = "type")]
    item_type: WatchableType,
    movie: Option<Movie>,
    episode: Option<Episode>,
}
