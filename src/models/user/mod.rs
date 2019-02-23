//! All models related to [users]
//!
//! [users]: https://trakt.docs.apiary.io/#reference/users
#[cfg(feature = "async")]
mod asyn;
#[cfg(feature = "sync")]
mod sync;

use crate::models::{Ids, ToId};
use chrono::{DateTime, Utc};

/// An [user]
///
/// [user]: https://trakt.docs.apiary.io/#reference/users
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub private: bool,
    pub name: Option<String>,
    pub vip: Option<bool>,
    pub vip_ep: Option<bool>,
    pub ids: Ids,
}

impl<'a> ToId<'a, &'a str> for User {
    fn id(&'a self) -> &'a str {
        self.ids.slug.as_ref().unwrap()
    }
}

/// Images of an [User]
///
/// [User]: struct.User.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserImages {
    pub avatar: Image,
}

/// An image with its url
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
}

/// An [User] with only optional fields
///
/// [User]: struct.User.html
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

/// An [user] with full [extended info]
///
/// [user]: https://trakt.docs.apiary.io/#reference/users
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
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

impl<'a> ToId<'a, &'a str> for FullUser {
    fn id(&'a self) -> &'a str {
        self.ids.slug.as_ref().unwrap()
    }
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

/// [Settings] of an user
///
/// [Settings]: https://trakt.docs.apiary.io/#reference/users/settings
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user: FullUser,
    pub account: SettingsAccount,
    pub connections: SettingsConnections,
    pub sharing_text: SettingsSharingText,
}

/// Account in [Settings]
///
/// [Settings]: struct.Settings.html
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccount {
    pub timezone: String,
    pub date_format: String,
    pub time_24hr: bool,
    pub cover_image: Option<String>,
    pub token: Option<String>,
}

/// Connections in [Settings]
///
/// [Settings]: struct.Settings.html
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConnections {
    pub facebook: bool,
    pub twitter: bool,
    pub google: bool,
    pub tumblr: bool,
    pub medium: bool,
    pub slack: bool,
}

/// Sharing text in [Settings]
///
/// [Settings]: struct.Settings.html
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsSharingText {
    pub watching: String,
    pub watched: String,
}

/// A [follow request]
///
/// [follow request]: https://trakt.docs.apiary.io/#reference/users/follower-requests
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequest {
    pub id: u32,
    pub requested_at: DateTime<Utc>,
    pub user: User,
}

/// Response after [approving a follow request]
///
/// [approving a follow request]: https://trakt.docs.apiary.io/#reference/users/approve-or-deny-follower-requests/approve-follow-request
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequestApprove {
    pub followed_at: DateTime<Utc>,
    pub user: User,
}
