use crate::filters::TypeFilter;
use crate::models::item_types::IncludeReplies;
use crate::models::AllCommentableItemType;
use crate::models::{
    list::OptionList, user::User, CommentableItemType, Episode, List, Movie, OptionEpisode,
    OptionMovie, OptionSeason, OptionShow, Season, Show,
};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum CommentType {
    Reviews,
    Shouts,
    All,
}

impl Display for CommentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            CommentType::Reviews => "reviews",
            CommentType::Shouts => "shouts",
            _ => "all",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentNew {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie: Option<OptionMovie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<OptionShow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<OptionSeason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<OptionEpisode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<OptionList>,
    pub comment: String,
    pub spoiler: bool,
    pub sharing: CommentSharing,
}

impl CommentNew {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentPost {
    pub comment: String,
    pub spoiler: bool,
}

impl CommentPost {
    pub fn new(comment: String, spoiler: bool) -> Self {
        Self { comment, spoiler }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
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
