use crate::models::{Episode, Movie, Show, WatchableType};
use chrono::{DateTime, Utc};

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
