use crate::{
    error::Error,
    models::{CalendarMovie, CalendarShow},
    TraktApi,
};
use chrono::{Date, Utc};

impl TraktApi {
    pub fn calendar_all_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_url!((
            "calendars",
            "all",
            "shows",
            start_date.format("%Y-%m-%d"),
            days
        )))
    }

    pub fn calendar_my_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
        access_token: String
    ) -> Result<Vec<CalendarShow>, Error> {
        self.auth_get(api_url!((
            "calendars",
            "my",
            "shows",
            start_date.format("%Y-%m-%d"),
            days
        )), access_token)
    }

    pub fn calendar_all_new_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_url!((
            "calendars",
            "all",
            "shows",
            "new",
            start_date.format("%Y-%m-%d"),
            days
        )))
    }

    pub fn calendar_my_new_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
        access_token: String
    ) -> Result<Vec<CalendarShow>, Error> {
        self.auth_get(api_url!((
            "calendars",
            "my",
            "shows",
            "new",
            start_date.format("%Y-%m-%d"),
            days
        )), access_token)
    }

    pub fn calendar_all_season_premieres(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_url!((
            "calendars",
            "all",
            "shows",
            "premieres",
            start_date.format("%Y-%m-%d"),
            days
        )))
    }

    pub fn calendar_my_season_premieres(
        &self,
        start_date: Date<Utc>,
        days: u32,
        access_token: String,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.auth_get(api_url!((
            "calendars",
            "my",
            "shows",
            "premieres",
            start_date.format("%Y-%m-%d"),
            days
        )), access_token)
    }

    pub fn calendar_all_movies(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.get(api_url!((
            "calendars",
            "all",
            "movies",
            start_date.format("%Y-%m-%d"),
            days
        )))
    }

    pub fn calendar_my_movies(
        &self,
        start_date: Date<Utc>,
        days: u32,
        access_token: String
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.auth_get(api_url!((
            "calendars",
            "my",
            "movies",
            start_date.format("%Y-%m-%d"),
            days
        )), access_token)
    }

    pub fn calendar_all_dvd(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.get(api_url!((
            "calendars",
            "all",
            "dvd",
            start_date.format("%Y-%m-%d"),
            days
        )))
    }

    pub fn calendar_my_dvd(
        &self,
        start_date: Date<Utc>,
        days: u32,
        access_token: String,
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.auth_get(api_url!((
            "calendars",
            "my",
            "dvd",
            start_date.format("%Y-%m-%d"),
            days
        )), access_token)
    }
}
