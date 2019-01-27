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
    pub liked_at: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLike {
    pub liked_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub item_type: LikeableType,
    pub comment: Option<Comment>,
    pub list: Option<List>,
}
