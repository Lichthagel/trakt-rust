#[derive(Debug, Serialize, Deserialize)]
pub struct Ids {
    trakt: Option<u64>,
    slug: Option<String>,
    tvdb: Option<u64>,
    imdb: Option<String>,
    tmdb: Option<u64>,
    tvrage: Option<u64>,
}
