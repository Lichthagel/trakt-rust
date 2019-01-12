use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    season::Season,
    item_type::ItemType,
    show::Show,
    movie::Movie,
};

pub struct Rating {
    rated_at: DateTime<Utc>,
    rating: u8,
    item_type: ItemType,
    season: Option<Season>,
    show: Option<Show>,
    movie: Option<Movie>,
}