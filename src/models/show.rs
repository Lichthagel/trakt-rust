use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    title: String,
    year: Option<u16>,
    ids: Ids,
}
