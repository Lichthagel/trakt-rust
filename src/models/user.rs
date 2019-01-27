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
    user: FullUser,
    account: SettingsAccount,
    connections: SettingsConnections,
    sharing_text: SettingsSharingText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccount {
    timezone: String,
    date_format: String,
    time_24hr: bool,
    cover_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConnections {
    facebook: bool,
    twitter: bool,
    google: bool,
    tumblr: bool,
    medium: bool,
    slack: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsSharingText {
    watching: String,
    watched: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequest {
    id: u32,
    requested_at: DateTime<Utc>,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequestApprove {
    followed_at: DateTime<Utc>,
    user: User,
}
