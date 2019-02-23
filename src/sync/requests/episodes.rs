use crate::{
    models::{
        Comment, Episode, FullEpisode, FullUser, List, ListFactory, MediaStats, Ratings,
        Translation, User,
    },
    sync::pagination::PaginationRequest,
    Result, TraktApi,
};
use reqwest::Method;
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn episode(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Episode> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number
        )))
    }

    pub fn episode_full(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<FullEpisode> {
        self.get(api_url!(
            (
                "shows",
                show_id,
                "seasons",
                season_number,
                "episodes",
                episode_number
            ),
            ("extended", "full")
        ))
    }

    pub fn episode_translations(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
        language: impl Display,
    ) -> Result<Vec<Translation>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "translations",
            language
        )))
    }

    pub fn episode_comments(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> PaginationRequest<Comment> {
        PaginationRequest::new(
            self,
            self.builder(
                Method::GET,
                api_url!((
                    "shows",
                    show_id,
                    "seasons",
                    season_number,
                    "episodes",
                    episode_number,
                    "comments"
                )),
            ),
        )
    }

    pub fn episode_lists(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> PaginationRequest<List> {
        let list_factory = f(ListFactory::default());

        PaginationRequest::new(
            self,
            self.builder(
                Method::GET,
                api_url!((
                    "shows",
                    show_id,
                    "seasons",
                    season_number,
                    "episodes",
                    episode_number,
                    "lists",
                    list_factory.list_filter,
                    list_factory.sorting
                )),
            ),
        )
    }

    pub fn episode_ratings(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Ratings> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "ratings"
        )))
    }

    pub fn episode_stats(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<MediaStats> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "stats"
        )))
    }

    pub fn episode_watching(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Vec<User>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "watching"
        )))
    }

    pub fn episode_watching_full(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Vec<FullUser>> {
        self.get(api_url!(
            (
                "shows",
                show_id,
                "seasons",
                season_number,
                "episodes",
                episode_number,
                "watching"
            ),
            ("extended", "full")
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::Error,
        models::{
            Comment, Episode, FullEpisode, Ids, List, ListFactory, ListFilter, ListSort,
            Translation, User,
        },
        pagination::Pagination,
        tests::mock,
        TraktApi,
    };
    use chrono::{offset::TimeZone, Utc};

    #[test]
    fn episode() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/3/episodes/3", "...")
            .with_status(200)
            .with_body_from_file("mock_data/episode.json")
            .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode("fairy-tail", 3, 3)
            .map(|res| {
                assert_eq!(
                    res,
                    Episode {
                        season: 3,
                        number: 3,
                        title: Some("Natsu vs. Gildarts".to_owned()),
                        ids: Ids {
                            trakt: Some(916320),
                            slug: None,
                            tvdb: Some(4173728),
                            imdb: None,
                            tmdb: Some(908194),
                            tvrage: Some(0),
                        },
                    }
                )
            })
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_full() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/3/episodes/3?extended=full",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_full.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_full("fairy-tail", 3, 3)
            .map(|res| {
                assert_eq!(
                    dbg!(res),
                    FullEpisode {
                        season: 3,
                        number: 3,
                        title: Some("Natsu vs. Gildarts".to_owned()),
                        ids: Ids {
                            trakt: Some(916320),
                            slug: None,
                            tvdb: Some(4173728),
                            imdb: None,
                            tmdb: Some(908194),
                            tvrage: Some(0),
                        },
                        number_abs: Some(99),
                        overview: Some("For their preliminary trials, Juvia and Lisanna face off against Erza, while Elfman and Evergreen find themselves standing against Mirajane. Meanwhile, Natsu enthusiastically battles against Gildarts, with a series of flashbacks detailing how Natsu had never once been able to defeat him since childhood. Just when Natsu believes he has gained the upper hand, Gildarts unleashes an immense aura of magical power, prompting Natsu to surrender in fear. Gildarts teaches Natsu the benefits that fear has in order for him to grow stronger, and tells him that he has passed his preliminary trial.".to_owned()),
                        rating: 8.55914,
                        votes: 93,
                        comment_count: 0,
                        first_aired: Some(Utc.ymd(2011, 9, 30).and_hms_milli(22, 0, 0, 0)),
                        updated_at: None,
                        available_translations: vec![
                            "ar".to_owned(),
                            "cs".to_owned(),
                            "de".to_owned(),
                            "el".to_owned(),
                            "en".to_owned(),
                            "es".to_owned(),
                            "fr".to_owned(),
                            "he".to_owned(),
                            "hu".to_owned(),
                            "it".to_owned(),
                            "ja".to_owned(),
                            "pt".to_owned(),
                            "ru".to_owned(),
                            "sr".to_owned(),
                            "uk".to_owned(),
                            "zh".to_owned()
                        ],
                        runtime: 24,
                    }
                )
            })
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_translations() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/3/episodes/3/translations/de",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_translations.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_translations("fairy-tail", 3, 3, "de")
            .map(|res| {
                assert_eq!(res, vec![Translation {
                    title: "Natsu versus Gildarts".to_owned(),
                    overview: "For their preliminary trials, Juvia and Lisanna face off against Erza, while Elfman and Evergreen find themselves standing against Mirajane. Meanwhile, Natsu enthusiastically battles against Gildarts, with a series of flashbacks detailing how Natsu had never once been able to defeat him since childhood. Just when Natsu believes he has gained the upper hand, Gildarts unleashes an immense aura of magical power, prompting Natsu to surrender in fear. Gildarts teaches Natsu the benefits that fear has in order for him to grow stronger, and tells him that he has passed his preliminary trial.".to_owned(),
                    tagline: None,
                    language: "de".to_owned(),
                }])
            })
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_comments() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/8/episodes/1/comments?page=1&limit=20",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_comments.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_comments("fairy-tail", 8, 1)
            .page(1)
            .limit(20)
            .execute()
            .map(|res| {
                assert!(res.contains(&Comment {
                    id: 194915,
                    parent_id: 0,
                    created_at: Utc.ymd(2018, 10, 10).and_hms(21, 12, 58),
                    updated_at: None,
                    comment: "don't know why i keep coming to watch this show, hoping it will be better, but as always it's full of...... \nyet another filler, you start a new season with filler what the hell is wrong with them".to_string(),
                    spoiler: false,
                    review: false,
                    replies: 0,
                    likes: 0,
                    user_rating: Some(1),
                    user: User {
                        username: "devilzeyez".to_string(),
                        private: false,
                        name: Some("Zeid Al - Dahabi".to_owned()),
                        vip: Some(false),
                        vip_ep: Some(false),
                        ids: Ids {
                            trakt: None,
                            slug: Some("devilzeyez".to_owned()),
                            tvdb: None,
                            imdb: None,
                            tmdb: None,
                            tvrage: None,
                        },
                    },
                }))
            })
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_lists() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/1/episodes/1/lists/all/added?page=1&limit=20",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_lists.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_lists("fairy-tail", 1, 1, |lf: ListFactory| {
                lf.with_sorting(ListSort::Added)
                    .with_filter_type(ListFilter::All)
            })
            .page(1)
            .limit(20)
            .execute()
            .map(|res| {
                assert!(res.contains(&List {
                    name: "w a t c h l i s t  - 2018".to_owned(),
                    description: Some("1st episode - MixNine \"Episode 6\"\r\n10th episode - Masterchef S7E10 |  \"The Weakest Link\"\r\n50th episode - Orphan Black S3E07 | \"Community of Dreadful Fear and Hate\"\r\n100th episode - I'm Not a Robot S1E15 | \"I'm Not a Robot\"\r\n150th episode - The Flash S1E23 | \"Fast Enough\"\r\n200th episode - What's Wrong With Secretary Kim? S1E10 | \"It Was You All Along\"\r\n250th episode - Strong Woman Do Bong Soon S1E16 | \"Final\"".to_owned()),
                    privacy: Some("public".to_owned()),
                    display_numbers: false,
                    allow_comments: true,
                    sort_by: "added".to_owned(),
                    sort_how: "asc".to_owned(),
                    created_at: Utc.ymd(2018, 01, 06).and_hms(16, 08, 55),
                    updated_at: None,
                    item_count: 287,
                    comment_count: 0,
                    likes: 0,
                    ids: Ids {
                        trakt: Some(4454156),
                        slug: Some("w-a-t-c-h-l-i-s-t-2018".to_owned()),
                        tvdb: None,
                        imdb: None,
                        tmdb: None,
                        tvrage: None,
                    },
                    user: User {
                        username: "w i n g s".to_owned(),
                        private: false,
                        name: Some("".to_owned()),
                        vip: Some(false),
                        vip_ep: Some(false),
                        ids: Ids {
                            trakt: None,
                            slug: Some("w-i-n-g-s".to_owned()),
                            tvdb: None,
                            imdb: None,
                            tmdb: None,
                            tvrage: None,
                        },
                    },
                }))
            })
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_ratings() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/1/episodes/1/ratings",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_ratings.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_ratings("fairy-tail", 1, 1)
            .map(|_res| ())
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_stats() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/1/episodes/1/stats", "...")
            .with_status(200)
            .with_body_from_file("mock_data/episode_stats.json")
            .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_stats("fairy-tail", 1, 1)
            .map(|_res| ())
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_watching() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/1/episodes/1/watching",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_watching.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_watching("fairy-tail", 1, 1)
            .map(|_res| ())
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }

    #[test]
    fn episode_watching_full() -> Result<(), Error> {
        let m = mock(
            "GET",
            "/shows/fairy-tail/seasons/1/episodes/1/watching?extended=full",
            "...",
        )
        .with_status(200)
        .with_body_from_file("mock_data/episode_watching_full.json")
        .create();

        TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .episode_watching_full("fairy-tail", 1, 1)
            .map(|_res| ())
            .and_then(|_| {
                m.assert();
                Ok(())
            })
    }
}
