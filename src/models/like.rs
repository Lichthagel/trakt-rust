use crate::models::{Comment, List, User};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LikeableType {
    Comment,
    List,
}

impl Display for LikeableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            LikeableType::Comment => "comments",
            LikeableType::List => "lists",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    liked_at: DateTime<Utc>,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLike {
    liked_at: DateTime<Utc>,
    #[serde(rename = "type")]
    item_type: LikeableType,
    comment: Option<Comment>,
    list: Option<List>,
}
