use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    ids: Ids
}