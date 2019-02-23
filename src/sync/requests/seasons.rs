use crate::{
    models::{Comment, Episode, List, ListFactory, MediaStats, Ratings, Season, User},
    Result, TraktApi,
};
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn seasons(&self, show_id: impl Display) -> Result<Vec<Season>> {
        self.get(api_url!(("shows", show_id, "seasons")))
    }

    pub fn season(&self, show_id: impl Display, season_number: u32) -> Result<Vec<Episode>> {
        self.get(api_url!(("shows", show_id, "seasons", season_number)))
    }

    pub fn season_comments(
        &self,
        show_id: impl Display,
        season_number: u32,
    ) -> Result<Vec<Comment>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "comments"
        )))
    }

    pub fn season_lists(
        &self,
        show_id: impl Display,
        season_number: u32,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "lists",
            list_factory.list_filter,
            list_factory.sorting
        )))
    }

    pub fn season_ratings(&self, show_id: impl Display, season_number: u32) -> Result<Ratings> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "ratings"
        )))
    }

    pub fn season_stats(&self, show_id: impl Display, season_number: u32) -> Result<MediaStats> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "stats"
        )))
    }

    pub fn season_watching(&self, show_id: impl Display, season_number: u32) -> Result<Vec<User>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "watching"
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::Error,
        models::{Comment, Episode, Ids, List, ListFilter, ListSort, Season, User},
        tests::mock,
        TraktApi,
    };
    use chrono::{offset::TimeZone, Utc};

    #[test]
    fn seasons() -> Result<(), Error> {
        let m = mock("GET", "/shows/the-expanse/seasons", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/seasons.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .seasons("the-expanse")
            .map(|res| {
                assert!(res.contains(&Season {
                    number: 3,
                    ids: Ids {
                        trakt: Some(152369),
                        slug: None,
                        tvdb: Some(750521),
                        imdb: None,
                        tmdb: Some(99857),
                        tvrage: None
                    }
                }))
            });

        m.assert();
        res
    }

    #[test]
    fn season() -> Result<(), Error> {
        let m = mock("GET", "/shows/the-expanse/seasons/3", "CLIENT_ID")
            .with_status(200)
            .with_body_from_file("mock_data/season.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .season("the-expanse", 3)
            .map(|res| {
                assert!(res.contains(&Episode {
                    season: 3,
                    number: 1,
                    title: Some("Fight or Flight".to_owned()),
                    ids: Ids {
                        trakt: Some(2758238),
                        slug: None,
                        tvdb: Some(6539689),
                        imdb: Some("tt6665296".to_owned()),
                        tmdb: Some(1438886),
                        tvrage: Some(0)
                    }
                }))
            });

        m.assert();
        res
    }
    #[test]
    fn season_comments() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/8/comments", "...")
            .with_status(200)
            .with_body_from_file("mock_data/media_comments.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .season_comments("fairy-tail", 8)
            .map(|res| {
                println!("{:#?}", res);
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
                            tvrage: None
                        }
                    }
                }))
            });

        m.assert();
        res
    }

    #[test]
    fn season_lists() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/1/lists/all/added", "...")
            .with_status(200)
            .with_body_from_file("mock_data/media_lists.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .season_lists("fairy-tail", 1, |lf| {
                lf.with_sorting(ListSort::Added)
                    .with_filter_type(ListFilter::All)
            })
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
                        tvrage: None
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
                            tvrage: None
                        }
                    }
                }))
            });

        m.assert();
        res
    }

    #[test]
    fn season_ratings() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/1/ratings", "...")
            .with_status(200)
            .with_body_from_file("mock_data/media_ratings.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .season_ratings("fairy-tail", 1)
            .map(|_res| ());

        m.assert();
        res
    }

    #[test]
    fn season_stats() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/1/stats", "...")
            .with_status(200)
            .with_body_from_file("mock_data/media_stats.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .season_stats("fairy-tail", 1)
            .map(|_res| ());

        m.assert();
        res
    }

    #[test]
    fn season_watching() -> Result<(), Error> {
        let m = mock("GET", "/shows/fairy-tail/seasons/1/watching", "...")
            .with_status(200)
            .with_body_from_file("mock_data/media_watching.json")
            .create();

        let res = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .season_watching("fairy-tail", 1)
            .map(|_res| ());

        m.assert();
        res
    }
}
