pub mod calendar_request;

pub use crate::sync::requests::calendar::calendar_request::CalendarRequest;

use crate::{
    models::{CalendarMovie, CalendarShow},
    TraktApi,
};

impl<'b> TraktApi<'b> {
    pub fn calendar_all_shows(&self) -> CalendarRequest<CalendarShow> {
        CalendarRequest::new(self, "all/shows", None)
    }

    pub fn calendar_my_shows<'a>(
        &'a self,
        access_token: &'a str,
    ) -> CalendarRequest<'a, CalendarShow> {
        CalendarRequest::new(self, "my/shows", Some(access_token))
    }

    pub fn calendar_all_new_shows(&self) -> CalendarRequest<CalendarShow> {
        CalendarRequest::new(self, "all/shows/new", None)
    }

    pub fn calendar_my_new_shows<'a>(
        &'a self,
        access_token: &'a str,
    ) -> CalendarRequest<'a, CalendarShow> {
        CalendarRequest::new(self, "my/shows/new", Some(access_token))
    }

    pub fn calendar_all_season_premieres(&self) -> CalendarRequest<CalendarShow> {
        CalendarRequest::new(self, "all/shows/premieres", None)
    }

    pub fn calendar_my_season_premieres<'a>(
        &'a self,
        access_token: &'a str,
    ) -> CalendarRequest<'a, CalendarShow> {
        CalendarRequest::new(self, "my/shows/premieres", Some(access_token))
    }

    pub fn calendar_all_movies(&self) -> CalendarRequest<CalendarMovie> {
        CalendarRequest::new(self, "all/movies", None)
    }

    pub fn calendar_my_movies<'a>(
        &'a self,
        access_token: &'a str,
    ) -> CalendarRequest<'a, CalendarMovie> {
        CalendarRequest::new(self, "my/movies", Some(access_token))
    }

    pub fn calendar_all_dvd(&self) -> CalendarRequest<CalendarMovie> {
        CalendarRequest::new(self, "all/dvd", None)
    }

    pub fn calendar_my_dvd<'a>(
        &'a self,
        access_token: &'a str,
    ) -> CalendarRequest<'a, CalendarMovie> {
        CalendarRequest::new(self, "my/dvd", Some(access_token))
    }
}
