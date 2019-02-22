//! All models related to [calendars]
//!
//! [calendars]: https://trakt.docs.apiary.io/#reference/calendars
use crate::{
    extended_info::{WithFull, WithNone},
    models::{Episode, FullEpisode, FullMovie, FullShow, Movie, Show},
};
use chrono::{DateTime, NaiveDate, Utc};

/// A show & episode in a [calendar]
///
/// [calendar]: https://trakt.docs.apiary.io/#reference/calendars/all-shows/get-shows
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarShow {
    pub first_aired: DateTime<Utc>,
    pub episode: Episode,
    pub show: Show,
}

/// A show & episode in a [calendar] with full [extended info]
///
/// [calendar]: https://trakt.docs.apiary.io/#reference/calendars
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCalendarShow {
    pub first_aired: DateTime<Utc>,
    pub episode: FullEpisode,
    pub show: FullShow,
}

impl WithFull for CalendarShow {
    type Full = FullCalendarShow;
}

impl WithNone for FullCalendarShow {
    type None = CalendarShow;
}

/// A movie in a [calendar]
///
/// [calendar]: https://trakt.docs.apiary.io/#reference/calendars
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMovie {
    pub released: NaiveDate,
    pub movie: Movie,
}

/// A movie in a [calendar] with full [extended info]
///
/// [calendar]: https://trakt.docs.apiary.io/#reference/calendars
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCalendarMovie {
    pub released: NaiveDate,
    pub movie: FullMovie,
}

impl WithFull for CalendarMovie {
    type Full = FullCalendarMovie;
}

impl WithNone for FullCalendarMovie {
    type None = CalendarMovie;
}
