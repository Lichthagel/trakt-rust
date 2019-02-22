use crate::models::{Comment, List, User};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;

/// All items that can be liked (not rated)
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

/// A [like] by a known comment/list
///
/// [like]: https://trakt.docs.apiary.io/#reference/comments/likes
#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    pub liked_at: DateTime<Utc>,
    pub user: User,
}

/// A [like] by a known user
///
/// [like]: https://trakt.docs.apiary.io/#reference/users/likes/get-likes
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLike {
    pub liked_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub item_type: LikeableType,
    pub comment: Option<Comment>,
    pub list: Option<List>,
}
