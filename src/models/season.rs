use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    number: u32,
    ids: Ids
}