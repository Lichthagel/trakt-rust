use crate::models::ids::Ids;

#[derive(Deserialize)]
pub struct Show {
    title: String,
    year: u16,
    ids: Ids
}