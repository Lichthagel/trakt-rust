use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}
