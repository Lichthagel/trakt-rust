use chrono::{
    DateTime,
    Utc
};
use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    liked_at: DateTime<Utc>,
    user: User
}