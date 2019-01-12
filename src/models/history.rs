use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    movie::Movie,
    item_type::ItemType,
    episode::Episode,
};

pub struct HistoryItem {
    id: u64,
    watched_at: DateTime<Utc>,
    action: String,
    content_type: ItemType,
    movie: Option<Movie>,
    episode: Option<Episode>,
}