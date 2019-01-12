pub struct Comment {
    id: u64,
    parent_id: u64,
    created_at: DateTime,
    updated_at: Option<DateTime>,
    comment: String,
    spoiler: bool,
    review: bool,
    replies: u64,
    likes: u64,
    user_rating: u8,
    user: User
}