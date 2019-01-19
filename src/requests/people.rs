use crate::{
    error::{Error, Result},
    models::{Credits, List, Person},
    TraktApi,
};

impl TraktApi {
    pub fn people(&self, id: String) -> Result<Person> {
        self.get(api_url!(("people", id)))
    }

    pub fn people_movie_credits(&self, id: String) -> Result<Credits> {
        self.get(api_url!(("people", id, "movies")))
    }

    pub fn people_show_credits(&self, id: String) -> Result<Credits> {
        self.get(api_url!(("people", id, "shows")))
    }

    // Todo add the possabillity to use filter
    pub fn people_lists(&self, id: String) -> Result<Vec<List>> {
        self.get(api_url!(("people", id, "lists")))
    }
}
