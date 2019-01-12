use chrono::{
    DateTime,
    Utc,
};
use crate::models::{
    item_type::ItemType,
    episode::Episode,
    show::Show,
    movie::Movie,
};

pub struct Watching {
    expires_at: DateTime<Utc>,
    started_at: DateTime<Utc>,
    action: String,
    item_type: ItemType,
    episode: Option<Episode>,
    show: Option<Show>,
    movie: Option<Movie>,
}