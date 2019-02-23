use crate::{
    models::{IdType, SearchItemType, SearchResult, SearchType},
    sync::pagination::PaginationRequest,
    Result, TraktApi,
};
use reqwest::Method;
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn search(&self, item_type: SearchType, query: &str) -> PaginationRequest<SearchResult> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("search", item_type)))
                .query(&[("query", query)]),
        )
    }

    pub fn id_lookup(
        &self,
        id_type: IdType,
        id: impl Display,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        self.get(api_url!(
            ("search", id_type, id),
            (
                "type",
                match item_type {
                    Some(t) => t.to_string(),
                    None => String::new(),
                }
            )
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::Error,
        models::{IdType, Ids, Movie, SearchItemType, SearchResult, SearchType},
        pagination::Pagination,
        tests::mock,
        TraktApi,
    };

    #[test]
    fn search() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/search/movie,show,?query=tron&page=1&limit=5",
            "CLIENT_ID",
        )
        .with_status(200)
        .with_body_from_file("mock_data/search.json")
        .create();

        let res = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .search(SearchType::new(true, true, false, false, false), "tron")
            .page(1)
            .limit(5)
            .execute()
            .map(|res| {
                assert!(res.contains(&SearchResult {
                    item_type: SearchItemType::Movie,
                    score: 1000.0,
                    movie: Some(Movie {
                        title: "Tron".to_string(),
                        year: Some(1982),
                        ids: Ids {
                            trakt: Some(66),
                            slug: Some("tron-1982".to_string()),
                            tvdb: None,
                            imdb: Some("tt0084827".to_string()),
                            tmdb: Some(97),
                            tvrage: None
                        }
                    }),
                    show: None,
                    episode: None,
                    person: None,
                    list: None
                }))
            });

        m.assert();
        res
    }

    #[test]
    fn id_lookup() -> Result<(), Error> {
        let m = mock("GET", "/search/imdb/tt0084827?type=movie", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/id_lookup.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .id_lookup(IdType::IMDb, "tt0084827", Some(SearchItemType::Movie))
            .map(|res| {
                assert_eq!(
                    res,
                    vec![SearchResult {
                        item_type: SearchItemType::Movie,
                        score: 1000.0,
                        movie: Some(Movie {
                            title: "Tron".to_string(),
                            year: Some(1982),
                            ids: Ids {
                                trakt: Some(66),
                                slug: Some("tron-1982".to_owned()),
                                tvdb: None,
                                imdb: Some("tt0084827".to_owned()),
                                tmdb: Some(97),
                                tvrage: None
                            }
                        }),
                        show: None,
                        episode: None,
                        person: None,
                        list: None
                    }]
                )
            });

        m.assert();
        res
    }

}
