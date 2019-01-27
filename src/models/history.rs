use crate::models::{Episode, Movie, WatchableType};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: u64,
    pub watched_at: DateTime<Utc>,
    pub action: String,
    #[serde(rename = "type")]
    pub item_type: WatchableType,
    pub movie: Option<Movie>,
    pub episode: Option<Episode>,
}
