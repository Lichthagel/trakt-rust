use crate::models::{user::User, CommentableItemType, Episode, List, Movie, Season, Show};
use chrono::{DateTime, Utc};

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
    user_rating: Option<u8>,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentItem {
    #[serde(rename = "type")]
    item_type: CommentableItemType,
    show: Option<Show>,
    movie: Option<Movie>,
    season: Option<Season>,
    episode: Option<Episode>,
    list: Option<List>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentAndItem {
    #[serde(rename = "type")]
    item_type: CommentableItemType,
    show: Option<Show>,
    movie: Option<Movie>,
    season: Option<Season>,
    episode: Option<Episode>,
    list: Option<List>,
    comment: Comment,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommentType {
    #[serde(rename = "reviews")]
    REVIEWS,
    #[serde(rename = "shouts")]
    SHOUTS,
    #[serde(rename = "all")]
    ALL,
}

impl ToString for CommentType {
    fn to_string(&self) -> String {
        String::from(match self {
            CommentType::REVIEWS => "reviews",
            CommentType::SHOUTS => "shouts",
            _ => "all",
        })
    }
}
