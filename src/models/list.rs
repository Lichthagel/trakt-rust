use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    Season,
    Episode,
    Movie,
    Show,
    Ids,
    Person,
    ListItemType
};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    name: String,
    description: String,
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
    ids: Ids
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
    person: Option<Person>
}