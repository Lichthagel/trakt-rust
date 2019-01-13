use chrono::{
    DateTime,
    Utc,
};
use crate::models::{
    Episode,
    Show,
    Movie,
    WatchableType
};

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