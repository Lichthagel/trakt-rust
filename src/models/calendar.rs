use crate::models::{Episode, Movie, Show};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarShow {
    first_aired: DateTime<Utc>,
    episode: Episode,
    show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMovie {
    released: NaiveDate,
    movie: Movie,
}
