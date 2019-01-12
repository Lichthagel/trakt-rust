use chrono::{
    DateTime,
    Utc
};
use crate::models::user::User;

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