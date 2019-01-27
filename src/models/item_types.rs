use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListItemType {
    Movie,
    Show,
    Season,
    Episode,
    Person,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentableItemType {
    Movie,
    Show,
    Season,
    Episode,
    List,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllCommentableItemType {
    Movie,
    Show,
    Season,
    Episode,
    List,
    All,
}

impl Display for AllCommentableItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            AllCommentableItemType::Movie => "movies",
            AllCommentableItemType::Show => "shows",
            AllCommentableItemType::Season => "seasons",
            AllCommentableItemType::Episode => "episodes",
            AllCommentableItemType::List => "lists",
            AllCommentableItemType::All => "all",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Movie,
    Show,
    Season,
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
#[serde(rename_all = "lowercase")]
pub enum AllItemType {
    Movie,
    Show,
    Season,
    Episode,
    All,
}

impl Display for AllItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            AllItemType::Movie => "movies",
            AllItemType::Show => "shows",
            AllItemType::Season => "seasons",
            AllItemType::Episode => "episodes",
            _ => "all",
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

#[derive(Debug, Serialize, Deserialize)]
pub enum IncludeReplies {
    True,
    False,
    Only,
}

impl Display for IncludeReplies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            IncludeReplies::True => "true",
            IncludeReplies::False => "false",
            IncludeReplies::Only => "only",
        })
    }
}
