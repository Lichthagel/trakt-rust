use crate::models::ids::Ids;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub private: bool,
    pub name: Option<String>,
    pub vip: Option<bool>,
    pub vip_ep: Option<bool>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserImages {
    pub avatar: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionUser {
    pub username: Option<String>,
    pub private: Option<bool>,
    pub name: Option<String>,
    pub vip: Option<bool>,
    pub vip_ep: Option<bool>,
    pub ids: Option<Ids>,
}

impl OptionUser {
    pub fn new(username: String) -> Self {
        Self {
            username: Some(username),
            private: None,
            name: None,
            vip: None,
            vip_ep: None,
            ids: None,
        }
    }
}

impl Default for OptionUser {
    fn default() -> Self {
        Self {
            username: None,
            private: None,
            name: None,
            vip: None,
            vip_ep: None,
            ids: None,
        }
    }
}

impl From<User> for OptionUser {
    fn from(user: User) -> Self {
        Self {
            username: Some(user.username),
            private: Some(user.private),
            name: user.name,
            vip: user.vip,
            vip_ep: user.vip_ep,
            ids: Some(user.ids),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullUser {
    pub username: String,
    pub private: bool,
    pub name: Option<String>,
    pub vip: Option<bool>,
    pub vip_ep: Option<bool>,
    pub ids: Ids,
    pub joined_at: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub about: Option<String>,
    pub gender: Option<String>,
    pub age: Option<u16>,
    pub images: Option<UserImages>,
    pub vip_og: Option<bool>,
    pub vip_years: Option<u32>,
}

impl PartialEq for FullUser {
    fn eq(&self, other: &FullUser) -> bool {
        self.username == other.username
            && self.private == other.private
            && self.name == other.name
            && self.vip == other.vip
            && self.vip_ep == other.vip_ep
            && self.ids == other.ids
            && self.joined_at == other.joined_at
    }
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
    pub cover_image: Option<String>,
    pub token: Option<String>,
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
