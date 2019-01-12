use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    season::Season,
    episode::Episode,
    movie::Movie,
    item_type::ItemType,
    show::Show,
};

pub struct ListEntry {
    rank: u32,
    listed_at: DateTime<Utc>,
    item_type: ItemType,
    movie: Option<Movie>,
    episode: Option<Episode>,
    season: Option<Season>,
    show: Option<Show>,
}