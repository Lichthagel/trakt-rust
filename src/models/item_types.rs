use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListItemType {
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "show")]
    SHOW,
    #[serde(rename = "season")]
    SEASON,
    #[serde(rename = "episode")]
    EPISODE,
    #[serde(rename = "person")]
    PERSON,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommentableItemType {
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "show")]
    SHOW,
    #[serde(rename = "season")]
    SEASON,
    #[serde(rename = "episode")]
    EPISODE,
    #[serde(rename = "list")]
    LIST,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllCommentableItemType {
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "show")]
    SHOW,
    #[serde(rename = "season")]
    SEASON,
    #[serde(rename = "episode")]
    EPISODE,
    #[serde(rename = "list")]
    LIST,
    #[serde(rename = "all")]
    ALL,
}

impl ToString for AllCommentableItemType {
    fn to_string(&self) -> String {
        String::from(match self {
            AllCommentableItemType::MOVIE => "movies",
            AllCommentableItemType::SHOW => "shows",
            AllCommentableItemType::SEASON => "seasons",
            AllCommentableItemType::EPISODE => "episodes",
            AllCommentableItemType::LIST => "lists",
            AllCommentableItemType::ALL => "all",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "season")]
    Season,
    #[serde(rename = "episode")]
    Episode,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ItemType::Movie => "movies",
            ItemType::Show => "shows",
            ItemType::Season => "seasons",
            ItemType::Episode => "episodes",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WatchableType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "episode")]
    Episode,
}

impl Display for WatchableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            WatchableType::Movie => "movies",
            WatchableType::Episode => "episodes",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "movies")]
    Movies,
    #[serde(rename = "shows")]
    Shows,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            MediaType::Movies => "movies",
            MediaType::Shows => "shows",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimePeriod {
    Weekly,
    Monthly,
    Yearly,
    All,
}

impl Display for TimePeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            TimePeriod::Weekly => "weekly",
            TimePeriod::Monthly => "monthly",
            TimePeriod::Yearly => "yearly",
            _ => "all",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchItemType {
    Movie,
    Show,
    Episode,
    Person,
    List,
}

impl Display for SearchItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            SearchItemType::Movie => "movie",
            SearchItemType::Show => "show",
            SearchItemType::Episode => "episode",
            SearchItemType::Person => "person",
            SearchItemType::List => "list",
        })
    }
}
