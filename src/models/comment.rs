use crate::models::{
    list::OptionList, user::User, CommentableItemType, Episode, List, Movie, OptionEpisode,
    OptionMovie, OptionSeason, OptionShow, Season, Show,
};
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
    twitter: bool,
    facebook: bool,
    tumblr: bool,
    medium: bool,
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
    comment: String,
    spoiler: bool,
}

impl CommentPost {
    pub fn new(comment: String, spoiler: bool) -> Self {
        Self { comment, spoiler }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
