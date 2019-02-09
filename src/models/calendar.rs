use crate::{
    extended_info::{WithFull, WithNone},
    models::{Episode, FullEpisode, FullMovie, FullShow, Movie, Show},
};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarShow {
    pub first_aired: DateTime<Utc>,
    pub episode: Episode,
    pub show: Show,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMovie {
    pub released: NaiveDate,
    pub movie: Movie,
}

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
