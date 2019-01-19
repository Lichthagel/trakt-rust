use crate::{
    error::Error,
    models::{
        Person,
        Credits,
        },
    TraktApi
};

impl TraktApi {
    pub fn people(&self, id: String) -> Result<Person, Error> {
        self.get(api_url!(("people", id)))
    }

    pub fn people_movie_credits(&self, id: String) -> Result<Credits, Error> {
        self.get(api_url!(("people", id, "movies")))
    }
}