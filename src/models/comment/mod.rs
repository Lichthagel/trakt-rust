//! All models related to [comments]
//!
//! [comments]: https://trakt.docs.apiary.io/#reference/comments
#[cfg(feature = "async")]
mod asyn;
#[cfg(feature = "sync")]
mod sync;

use crate::{
    extended_info::{WithFull, WithNone},
    filters::TypeFilter,
    models::{
        AllCommentableItemType, CommentableItemType, Episode, FullEpisode, FullList, FullMovie,
        FullSeason, FullShow, FullUser, IncludeReplies, List, Movie, Season, Show, ToId, User,
    },
};
use chrono::{DateTime, Utc};
use std::fmt;

/// A [comment]
///
/// [comment]: https://trakt.docs.apiary.io/#reference/comments
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub parent_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub comment: String,
    pub spoiler: bool,
    pub review: bool,
    pub replies: u64,
    pub likes: u64,
    pub user_rating: Option<u8>,
    pub user: User,
}

impl ToId<u32> for Comment {
    fn id(&self) -> u32 {
        self.id as u32
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Comment) -> bool {
        self.id == other.id
            && self.parent_id == other.parent_id
            && self.created_at == other.created_at
            && self.comment == other.comment
            && self.spoiler == other.spoiler
            && self.review == other.review
            && self.user == other.user
    }
}

/// A [comment] with full [extended info]
///
/// [comment]: https://trakt.docs.apiary.io/#reference/comments
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullComment {
    pub id: u64,
    pub parent_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub comment: String,
    pub spoiler: bool,
    pub review: bool,
    pub replies: u64,
    pub likes: u64,
    pub user_rating: Option<u8>,
    pub user: FullUser,
}

impl ToId<u32> for FullComment {
    fn id(&self) -> u32 {
        self.id as u32
    }
}

impl PartialEq for FullComment {
    fn eq(&self, other: &FullComment) -> bool {
        self.id == other.id
            && self.parent_id == other.parent_id
            && self.created_at == other.created_at
            && self.comment == other.comment
            && self.spoiler == other.spoiler
            && self.review == other.review
            && self.user == other.user
    }
}

impl WithFull for Comment {
    type Full = FullComment;
}

impl WithNone for FullComment {
    type None = Comment;
}

/// Wraps around an item that can be commented
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentItem {
    #[serde(rename = "type")]
    pub item_type: CommentableItemType,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
    pub season: Option<Season>,
    pub episode: Option<Episode>,
    pub list: Option<List>,
}

/// Wraps around an item that can be [comment]ed including the [comment]
///
/// [comment]: https://trakt.docs.apiary.io/#reference/comments
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentAndItem {
    #[serde(rename = "type")]
    pub item_type: CommentableItemType,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
    pub season: Option<Season>,
    pub episode: Option<Episode>,
    pub list: Option<List>,
    pub comment: Comment,
}

impl ToId<u32> for CommentAndItem {
    fn id(&self) -> u32 {
        self.comment.id()
    }
}

/// Wraps around an item that can be [comment]ed including the [comment] and full [extended info]
///
/// [comment]: https://trakt.docs.apiary.io/#reference/comments
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCommentAndItem {
    #[serde(rename = "type")]
    pub item_type: CommentableItemType,
    pub show: Option<FullShow>,
    pub movie: Option<FullMovie>,
    pub season: Option<FullSeason>,
    pub episode: Option<FullEpisode>,
    pub list: Option<FullList>,
    pub comment: FullComment,
}

impl ToId<u32> for FullCommentAndItem {
    fn id(&self) -> u32 {
        self.comment.id()
    }
}

impl WithFull for CommentAndItem {
    type Full = FullCommentAndItem;
}

impl WithNone for FullCommentAndItem {
    type None = CommentAndItem;
}

/// For requesting different types of [comment]s
///
/// [comment]: https://trakt.docs.apiary.io/#reference/comments
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum CommentType {
    Reviews,
    Shouts,
    All,
}

impl fmt::Display for CommentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            CommentType::Reviews => "reviews",
            CommentType::Shouts => "shouts",
            _ => "all",
        })
    }
}

/// For specifying where a comment should be shared
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentSharing {
    pub twitter: bool,
    pub facebook: bool,
    pub tumblr: bool,
    pub medium: bool,
}

impl CommentSharing {
    pub fn new(twitter: bool, facebook: bool, tumblr: bool, medium: bool) -> Self {
        Self {
            twitter,
            facebook,
            tumblr,
            medium,
        }
    }
}

pub struct GetComments {
    pub comment_type: CommentType,
    pub item_type: AllCommentableItemType,
    pub include_replies: IncludeReplies,
}

impl GetComments {
    pub fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.comment_type = comment_type;
        self
    }

    pub fn include_replies(mut self, include_replies: IncludeReplies) -> Self {
        self.include_replies = include_replies;
        self
    }
}

impl TypeFilter<AllCommentableItemType> for GetComments {
    fn item_type(mut self, item_type: AllCommentableItemType) -> Self {
        self.item_type = item_type;
        self
    }
}

impl Default for GetComments {
    fn default() -> Self {
        GetComments {
            comment_type: CommentType::All,
            item_type: AllCommentableItemType::All,
            include_replies: IncludeReplies::False,
        }
    }
}
