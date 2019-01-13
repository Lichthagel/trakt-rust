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