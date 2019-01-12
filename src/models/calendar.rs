use chrono::{
    DateTime,
    Utc,
    Date
};
use crate::models::{
    episode::Episode,
    show::Show,
    movie::Movie,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarShow {
    first_aired: DateTime<Utc>,
    episode: Episode,
    show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarMovie {
    released: Date<Utc>,
    movie: Movie,
}