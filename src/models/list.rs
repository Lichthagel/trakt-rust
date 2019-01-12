pub struct ListEntry {
    rank: u32,
    listed_at: DateTime,
    item_type: ItemType,
    movie: Option<Movie>,
    episode: Option<Episode>,
    season: Option<Season>,
    show: Option<Show>
}