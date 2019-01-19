use crate::{
    error::Error,
    models::{Credits, List, Person},
    TraktApi,
};

impl TraktApi {
    pub fn people(&self, id: String) -> Result<Person, Error> {
        self.get(api_url!(("people", id)))
    }

    pub fn people_movie_credits(&self, id: String) -> Result<Credits, Error> {
        self.get(api_url!(("people", id, "movies")))
    }

    pub fn people_show_credits(&self, id: String) -> Result<Credits, Error> {
        self.get(api_url!(("people", id, "shows")))
    }

    // Todo add the possabillity to use filter
    pub fn people_lists(&self, id: String) -> Result<Vec<List>, Error> {
        self.get(api_url!(("people", id, "lists")))
    }
}
