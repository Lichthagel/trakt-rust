use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    Season,
    Show,
    Movie,
    MediaType
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    rated_at: DateTime<Utc>,
    rating: u8,
    #[serde(rename = "type")]
    item_type: MediaType,
    season: Option<Season>,
    show: Option<Show>,
    movie: Option<Movie>,
}