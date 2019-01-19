use crate::models::User;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    liked_at: DateTime<Utc>,
    user: User,
}
