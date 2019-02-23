use crate::{
    asyn::{Result, TraktApi},
    models::{Credits, List, ListFactory, Person},
};
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn people(&self, id: impl Display) -> Result<Person> {
        self.get(api_url!(("people", id)))
    }

    pub fn people_movie_credits(&self, id: impl Display) -> Result<Credits> {
        self.get(api_url!(("people", id, "movies")))
    }

    pub fn people_show_credits(&self, id: impl Display) -> Result<Credits> {
        self.get(api_url!(("people", id, "shows")))
    }

    pub fn people_lists(
        &self,
        id: impl Display,
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

#[cfg(test)]
mod tests {
    use crate::{
        asyn::TraktApi,
        error::Error,
        models::{Character, Ids, List, ListFilter, ListSort, Movie, Person, Show, User},
        tests::mock,
    };
    use chrono::{offset::TimeZone, Utc};
    use futures::future::Future;
    use tokio_core::reactor::Core;

    #[test]
    fn people() -> Result<(), Error> {
        let m = mock("GET", "/people/jeff-bridges", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/people.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .people("jeff-bridges")
            .map(|res| {
                assert_eq!(
                    res,
                    Person {
                        name: "Jeff Bridges".to_string(),
                        ids: Ids {
                            trakt: Some(4173),
                            slug: Some("jeff-bridges".to_string()),
                            tvdb: None,
                            imdb: Some("nm0000313".to_string()),
                            tmdb: Some(1229),
                            tvrage: Some(59067)
                        }
                    }
                )
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn people_movie_credits() -> Result<(), Error> {
        let m = mock("GET", "/people/jeff-bridges/movies", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/people_movie_credits.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .people_movie_credits("jeff-bridges")
            .map(|res| {
                assert!(res.cast.unwrap().contains(&Character {
                    character: "Kevin Flynn / Clu".to_string(),
                    show: None,
                    movie: Some(Movie {
                        title: "TRON: Legacy".to_string(),
                        year: Some(2010),
                        ids: Ids {
                            trakt: Some(12601),
                            slug: Some("tron-legacy-2010".to_string()),
                            tvdb: None,
                            imdb: Some("tt1104001".to_string()),
                            tmdb: Some(20526),
                            tvrage: None
                        }
                    })
                }))
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn people_show_credits() -> Result<(), Error> {
        let m = mock("GET", "/people/jeff-bridges/shows", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/people_show_credits.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .people_show_credits("jeff-bridges")
            .map(|res| {
                assert!(res.cast.unwrap().contains(&Character {
                    character: "Dave Melkin".to_string(),
                    show: Some(Show {
                        title: "The Lloyd Bridges Show".to_string(),
                        year: Some(1962),
                        ids: Ids {
                            trakt: Some(16129),
                            slug: Some("the-lloyd-bridges-show".to_string()),
                            tvdb: Some(77359),
                            imdb: None,
                            tmdb: Some(16199),
                            tvrage: Some(12820)
                        }
                    }),
                    movie: None
                }))
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }
    #[test]
    fn people_lists() -> Result<(), Error> {
        let m = mock("GET", "/people/jeff-bridges/lists/all/added", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/people_lists.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .people_lists("jeff-bridges", |lf| {
                lf.with_filter_type(ListFilter::All)
                    .with_sorting(ListSort::Added)
            })
            .map(|res| {
                assert!(res.contains(&List {
                    name: "Favourite Actors".to_string(),
                    description: Some("".to_string()),
                    privacy: Some("public".to_string()),
                    display_numbers: false,
                    allow_comments: true,
                    sort_by: "rank".to_string(),
                    sort_how: "asc".to_string(),
                    created_at: Utc.ymd(2018, 12, 19).and_hms(10, 26, 30),
                    updated_at: None,
                    item_count: 28,
                    comment_count: 0,
                    likes: 0,
                    ids: Ids {
                        trakt: Some(6013542),
                        slug: Some("favourite-actors".to_string()),
                        tvdb: None,
                        imdb: None,
                        tmdb: None,
                        tvrage: None
                    },
                    user: User {
                        username: "Kasady".to_string(),
                        private: false,
                        name: Some("Kasady".to_string()),
                        vip: Some(false),
                        vip_ep: Some(false),
                        ids: Ids {
                            trakt: None,
                            slug: Some("kasady".to_string()),
                            tvdb: None,
                            imdb: None,
                            tmdb: None,
                            tvrage: None
                        }
                    }
                }))
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

}
