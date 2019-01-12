pub struct HistoryItem {
    id: u64,
    watched_at: DateTime,
    action: String,
    content_type: ItemType,
    movie: Option<Movie>,
    episode: Option<Episode>
}