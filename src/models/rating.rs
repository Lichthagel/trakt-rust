use crate::models::{ItemType, Movie, Season, Show};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    rated_at: DateTime<Utc>,
    rating: u8,
    #[serde(rename = "type")]
    item_type: ItemType,
    season: Option<Season>,
    show: Option<Show>,
    movie: Option<Movie>,
}
