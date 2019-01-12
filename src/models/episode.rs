use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    season: u32,
    number: u32,
    title: Option<String>,
    ids: Ids
}