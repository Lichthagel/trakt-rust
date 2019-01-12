pub struct Rating {
    rated_at: DateTime,
    rating: u8,
    item_type: ItemType,
    season: Option<Season>,
    show: Option<Show>,
    movie: Option<Movie>
}