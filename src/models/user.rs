use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    private: bool,
    name: Option<String>,
    vip: Option<bool>,
    vip_ep: Option<bool>,
    ids: Ids
}