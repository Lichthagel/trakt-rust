use crate::{
    models::{Credits, List, ListFactory, Person},
    Result, TraktApi,
};

impl TraktApi {
    pub fn people(&self, id: &str) -> Result<Person> {
        self.get(api_url!(("people", id)))
    }

    pub fn people_movie_credits(&self, id: &str) -> Result<Credits> {
        self.get(api_url!(("people", id, "movies")))
    }

    pub fn people_show_credits(&self, id: &str) -> Result<Credits> {
        self.get(api_url!(("people", id, "shows")))
    }

    pub fn people_lists(
        &self,
        id: &str,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        self.get(api_url!((
            "people",
            id,
            "lists",
            list_factory.list_filter,
            list_factory.sorting
        )))
    }
}
