use crate::models::ids::Ids;

#[derive(Deserialize)]
pub struct Episode {
    season: u32,
    number: u32,
    title: String,
    ids: Ids
}