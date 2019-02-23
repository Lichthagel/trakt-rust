use crate::{
    asyn::{Result, TraktApi},
    models::{Movie, Show},
};
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn recommendations_movie(&self, access_token: &str) -> Result<Vec<Movie>> {
        self.auth_get(api_url!(("recommendations", "movies")), access_token)
    }

    pub fn recommendations_movie_hide(&self, id: impl Display, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("recommendations", "movies", id)), access_token)
    }

    pub fn recommendations_show(&self, access_token: &str) -> Result<Vec<Show>> {
        self.auth_get(api_url!(("recommendations", "shows")), access_token)
    }

    pub fn recommendations_show_hide(&self, id: impl Display, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("recommendations", "shows", id)), access_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        asyn::TraktApi,
        error::Error,
        models::{Ids, Movie, Show},
        tests::auth_mock,
    };
    use futures::future::Future;
    use mockito::server_url;
    use tokio_core::reactor::Core;

    #[test]
    fn recommendations_movie() -> Result<(), Error> {
        let m = auth_mock(
            "GET",
            "/recommendations/movies",
            "CLIENT_ID",
            "ACCESS_TOKEN",
        )
        .with_status(200)
        .with_body_from_file("mock_data/movies.json")
        .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_string(), None)
            .recommendations_movie("ACCESS_TOKEN")
            .map(|res| {
                assert!(res.contains(&Movie {
                    title: "The Revenant".to_string(),
                    year: Some(2015),
                    ids: Ids {
                        trakt: Some(179334),
                        slug: Some("the-revenant-2015".to_string()),
                        tvdb: None,
                        imdb: Some("tt1663202".to_string()),
                        tmdb: Some(281957),
                        tvrage: None
                    }
                }))
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn recommendations_movie_hide() -> Result<(), Error> {
        let m = auth_mock(
            "DELETE",
            "/recommendations/movies/the-revenant-2015",
            "CLIENT_ID",
            "ACCESS_TOKEN",
        )
        .with_status(204)
        .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_string(), None)
            .recommendations_movie_hide("the-revenant-2015", "ACCESS_TOKEN")
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn recommendations_show() -> Result<(), Error> {
        let m = auth_mock("GET", "/recommendations/shows", "CLIENT_ID", "ACCESS_TOKEN")
            .with_status(200)
            .with_body_from_file("mock_data/shows.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_string(), None)
            .recommendations_show("ACCESS_TOKEN")
            .map(|res| {
                assert!(res.contains(&Show {
                    title: "My Hero Academia".to_string(),
                    year: Some(2016),
                    ids: Ids {
                        trakt: Some(104311),
                        slug: Some("my-hero-academia".to_string()),
                        tvdb: Some(305074),
                        imdb: Some("tt5626028".to_string()),
                        tmdb: Some(65930),
                        tvrage: None
                    }
                }))
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn recommendations_show_hide() -> Result<(), Error> {
        let m = auth_mock(
            "DELETE",
            "/recommendations/shows/my-hero-academia",
            "CLIENT_ID",
            "ACCESS_TOKEN",
        )
        .with_status(204)
        .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_string(), None)
            .recommendations_show_hide("my-hero-academia", "ACCESS_TOKEN")
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

}
