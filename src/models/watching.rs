pub struct Watching {
    expires_at: DateTime,
    started_at: DateTime,
    action: String,
    item_type: ItemType,
    episode: Option<Episode>,
    show: Option<Show>,
    movie: Option<Movie>
}