use crate::{asyn::pagination::PaginationRequest, asyn::TraktApi, models::ListInfo};
use reqwest::Method;

impl<'a> TraktApi<'a> {
    pub fn lists_trending(&self) -> PaginationRequest<ListInfo> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("lists", "trending"))),
        )
    }

    pub fn lists_popular(&self) -> PaginationRequest<ListInfo> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("lists", "popular"))),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        asyn::TraktApi,
        error::Error,
        models::{Ids, List, ListInfo, User},
        pagination::Pagination,
        tests::mock,
    };
    use chrono::{offset::TimeZone, Utc};
    use futures::future::Future;
    use mockito::server_url;
    use tokio_core::reactor::Core;

    #[test]
    fn lists_trending() -> Result<(), Error> {
        let m = mock("GET", "/lists/trending?page=1&limit=5", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/lists.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_owned(), None)
            .lists_trending()
            .page(1)
            .limit(5)
            .execute()
            .map(|res| {
                assert!(
                    res.contains(
                        &ListInfo {
                            like_count: 89,
                            comment_count: 0,
                            list: List {
                                name: "Movie Selections Based on Subreddits".to_string(),
                                description: Some("This is a list based on movies found in the following subreddits: r/iwatchedanoldmovier/flicksr/moviesclub".to_string()),
                                privacy: Some("public".to_string()),
                                display_numbers: false,
                                allow_comments: true,
                                sort_by: "rank".to_string(),
                                sort_how: "asc".to_string(),
                                created_at: Utc.ymd(2019, 02, 18).and_hms(16, 37, 54),
                                updated_at: None,
                                item_count: 92,
                                comment_count: 0,
                                likes: 89,
                                ids: Ids {
                                    trakt: Some(6319578),
                                    slug: Some("movie-selections-based-on-subreddits".to_string()),
                                    tvdb: None,
                                    imdb: None,
                                    tmdb: None,
                                    tvrage: None
                                },
                                user: User {
                                    username: "Giladg".to_string(),
                                    private: false,
                                    name: Some("Gilad & Alex".to_string()),
                                    vip: Some(false),
                                    vip_ep: Some(false),
                                    ids: Ids {
                                        trakt: None,
                                        slug: Some("giladg".to_owned()),
                                        tvdb: None,
                                        imdb: None,
                                        tmdb: None,
                                        tvrage: None
                                    }
                                }
                            }
                        }
                    )
                )
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn lists_popular() -> Result<(), Error> {
        let m = mock("GET", "/lists/popular?page=1&limit=5", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/lists.json")
            .create();

        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&server_url(), "CLIENT_ID".to_owned(), None)
            .lists_popular()
            .page(1)
            .limit(5)
            .execute()
            .map(|res| {
                assert!(
                    res.contains(
                        &ListInfo {
                            like_count: 89,
                            comment_count: 0,
                            list: List {
                                name: "Movie Selections Based on Subreddits".to_string(),
                                description: Some("This is a list based on movies found in the following subreddits: r/iwatchedanoldmovier/flicksr/moviesclub".to_string()),
                                privacy: Some("public".to_string()),
                                display_numbers: false,
                                allow_comments: true,
                                sort_by: "rank".to_string(),
                                sort_how: "asc".to_string(),
                                created_at: Utc.ymd(2019, 02, 18).and_hms(16, 37, 54),
                                updated_at: None,
                                item_count: 92,
                                comment_count: 0,
                                likes: 89,
                                ids: Ids {
                                    trakt: Some(6319578),
                                    slug: Some("movie-selections-based-on-subreddits".to_string()),
                                    tvdb: None,
                                    imdb: None,
                                    tmdb: None,
                                    tvrage: None
                                },
                                user: User {
                                    username: "Giladg".to_string(),
                                    private: false,
                                    name: Some("Gilad & Alex".to_string()),
                                    vip: Some(false),
                                    vip_ep: Some(false),
                                    ids: Ids {
                                        trakt: None,
                                        slug: Some("giladg".to_owned()),
                                        tvdb: None,
                                        imdb: None,
                                        tmdb: None,
                                        tvrage: None
                                    }
                                }
                            }
                        }
                    )
                )
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

}
