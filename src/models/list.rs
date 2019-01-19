use crate::models::{Episode, Ids, ListItemType, Movie, Person, Season, Show, User};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    name: String,
    description: Option<String>,
    privacy: String,
    display_numbers: bool,
    allow_comments: bool,
    sort_by: String,
    sort_how: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    item_count: u64,
    comment_count: u64,
    likes: u64,
    ids: Ids,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListInfo {
    like_count: u32,
    comment_count: u32,
    list: List,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    rank: u32,
    listed_at: DateTime<Utc>,
    #[serde(rename = "type")]
    item_type: ListItemType,
    movie: Option<Movie>,
    episode: Option<Episode>,
    season: Option<Season>,
    show: Option<Show>,
    person: Option<Person>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListType {
    Personal,
    Official,
    Watchlists,
    All,
}

impl Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ListType::Personal => "personal",
            ListType::Official => "official",
            ListType::Watchlists => "watchlists",
            _ => "all",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListSort {
    Popular,
    Likes,
    Comments,
    Items,
    Added,
    Updated,
}

impl Display for ListSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ListSort::Popular => "popular",
            ListSort::Likes => "likes",
            ListSort::Comments => "comments",
            ListSort::Items => "items",
            ListSort::Added => "added",
            ListSort::Updated => "updated",
        })
    }
}
