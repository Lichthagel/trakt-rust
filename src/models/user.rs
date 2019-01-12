use crate::models::ids::Ids;

pub struct User {
    username: String,
    private: bool,
    name: String,
    vip: bool,
    vip_ep: bool,
    ids: Ids
}