pub mod authentication;
pub mod calendar;
pub mod certifications;
pub mod comment;
pub mod episode;
pub mod genre;
pub mod history;
pub mod ids;
pub mod item_type;
pub mod list;
pub mod movie;
pub mod person;
pub mod rating;
pub mod season;
pub mod show;
pub mod stats;
pub mod user;
pub mod watched;
pub mod watching;

pub use crate::models::{
    authentication::Authentication,
    calendar::{
        CalendarShow,
        CalendarMovie,
    },
    certifications::{
        CertificationsType,
        Certification,
        Certifications,
    },
    comment::Comment,
    episode::Episode,
    genre::Genre,
    history::HistoryItem,
    ids::Ids,
    item_type::ItemType,
    list::ListEntry,
    movie::Movie,
    person::Person,
    rating::Rating,
    season::Season,
    show::Show,
    stats::{
        Stats,
        RatingStats,
        SeasonStats,
        ShowStats,
        NetworkStats,
        MovieStats,
        EpisodeStats,
    },
    user::User,
    watched::{
        WatchedEpisode,
        WatchedEntry,
        WatchedSeason,
    },
    watching::Watching,
};
