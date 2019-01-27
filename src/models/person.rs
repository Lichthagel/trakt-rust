use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub ids: Ids,
}
