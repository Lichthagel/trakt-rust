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
    PERSON
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
    LIST
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
    ALL
}

impl ToString for AllCommentableItemType {
    fn to_string(&self) -> String {
        String::from(
            match self {
                AllCommentableItemType::MOVIE => "movies",
                AllCommentableItemType::SHOW => "shows",
                AllCommentableItemType::SEASON => "seasons",
                AllCommentableItemType::EPISODE => "episodes",
                AllCommentableItemType::LIST => "lists",
                AllCommentableItemType::ALL => "all"
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "show")]
    SHOW,
    #[serde(rename = "season")]
    SEASON,
    #[serde(rename = "episode")]
    EPISODE
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WatchableType {
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "episode")]
    EPISODE
}