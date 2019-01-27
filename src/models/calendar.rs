use crate::models::{Episode, Movie, Show};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarShow {
    pub first_aired: DateTime<Utc>,
    pub episode: Episode,
    pub show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMovie {
    pub released: NaiveDate,
    pub movie: Movie,
}
