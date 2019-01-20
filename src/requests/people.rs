use crate::{
    error::Result,
    models::{Credits, List, Person, PeopleListSearchFactory},
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

    pub fn people_lists<F>(&self, id: String, f: F) -> Result<Vec<List>>
    where
        F: FnOnce(PeopleListSearchFactory) -> PeopleListSearchFactory,
    {
        let search_factory = f(PeopleListSearchFactory::default());
        self.get(api_url!((
            "people",
            id,
            "lists",
            search_factory.filter_type,
            search_factory.sorting
        )))
    }
}
