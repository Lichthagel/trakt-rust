use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub season: u32,
    pub number: u32,
    pub title: Option<String>,
    pub ids: Ids,
}
