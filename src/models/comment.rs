use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    user::User,
    Show,
    Movie,
    Episode,
    Season,
    List,
    CommentableItemType
};
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    id: u64,
    parent_id: u64,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    comment: String,
    spoiler: bool,
    review: bool,
    replies: u64,
    likes: u64,
    user_rating: u8,
    user: User
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentItem {
    #[serde(rename = "type")]
    item_type: CommentableItemType,
    show: Option<Show>,
    movie: Option<Movie>,
    season: Option<Season>,
    episode: Option<Episode>,
    list: Option<List>
}