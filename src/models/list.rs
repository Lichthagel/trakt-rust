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
pub struct OptionList {
    pub name: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<String>,
    pub display_numbers: Option<bool>,
    pub allow_comments: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_how: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub item_count: Option<u64>,
    pub comment_count: Option<u64>,
    pub likes: Option<u64>,
    pub ids: Option<Ids>,
    pub user: Option<User>,
}

impl OptionList {
    pub fn new(trakt_id: u64) -> Self {
        Self {
            name: None,
            description: None,
            privacy: None,
            display_numbers: None,
            allow_comments: None,
            sort_by: None,
            sort_how: None,
            created_at: None,
            updated_at: None,
            item_count: None,
            comment_count: None,
            likes: None,
            ids: Some(Ids {
                trakt: Some(trakt_id),
                slug: None,
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            }),
            user: None,
        }
    }
}

impl From<List> for OptionList {
    fn from(list: List) -> Self {
        Self {
            name: Some(list.name),
            description: list.description,
            privacy: Some(list.privacy),
            display_numbers: Some(list.display_numbers),
            allow_comments: Some(list.allow_comments),
            sort_by: Some(list.sort_by),
            sort_how: Some(list.sort_how),
            created_at: Some(list.created_at),
            updated_at: list.updated_at,
            item_count: Some(list.item_count),
            comment_count: Some(list.comment_count),
            likes: Some(list.likes),
            ids: Some(list.ids),
            user: Some(list.user),
        }
    }
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
