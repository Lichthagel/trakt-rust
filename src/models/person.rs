use crate::models::ids::Ids;

pub struct Person {
    username: String,
    private: bool,
    name: String,
    vip: bool,
    vip_ep: bool,
    ids: Ids
}