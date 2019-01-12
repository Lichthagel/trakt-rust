use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    title: String,
    year: u16,
    ids: Ids
}