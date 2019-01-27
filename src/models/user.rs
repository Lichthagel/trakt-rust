use crate::models::ids::Ids;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    private: bool,
    name: Option<String>,
    vip: Option<bool>,
    vip_ep: Option<bool>,
    ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserImages {
    avatar: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    full: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullUser {
    username: String,
    private: bool,
    name: Option<String>,
    vip: Option<bool>,
    vip_ep: Option<bool>,
    ids: Ids,
    joined_at: DateTime<Utc>,
    location: String,
    about: Option<String>,
    gender: String,
    age: u16,
    images: UserImages,
    vip_og: Option<bool>,
    vip_years: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user: FullUser,
    pub account: SettingsAccount,
    pub connections: SettingsConnections,
    pub sharing_text: SettingsSharingText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccount {
    pub timezone: String,
    pub date_format: String,
    pub time_24hr: bool,
    pub cover_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConnections {
    pub facebook: bool,
    pub twitter: bool,
    pub google: bool,
    pub tumblr: bool,
    pub medium: bool,
    pub slack: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsSharingText {
    pub watching: String,
    pub watched: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequest {
    pub id: u32,
    pub requested_at: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequestApprove {
    pub followed_at: DateTime<Utc>,
    pub user: User,
}
