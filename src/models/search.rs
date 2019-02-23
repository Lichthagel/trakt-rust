//! All models related to [search]
//!
//! [search]: https://trakt.docs.apiary.io/#reference/search
use crate::models::{Episode, List, Movie, Person, SearchItemType, Show};
use std::fmt;
use std::fmt::Display;

/// A [search] result
///
/// [search]: https://trakt.docs.apiary.io/#reference/search
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    #[serde(rename = "type")]
    pub item_type: SearchItemType,
    pub score: f64,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub episode: Option<Episode>,
    pub person: Option<Person>,
    pub list: Option<List>,
}

/// Types to [search] after
///
/// [search]: https://trakt.docs.apiary.io/#reference/search
pub struct SearchType {
    pub movie: bool,
    pub show: bool,
    pub episode: bool,
    pub person: bool,
    pub list: bool,
}

impl SearchType {
    pub fn new(movie: bool, show: bool, episode: bool, person: bool, list: bool) -> Self {
        Self {
            movie,
            show,
            episode,
            person,
            list,
        }
    }
}

impl Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        if self.movie {
            s.push_str("movie,");
        }
        if self.show {
            s.push_str("show,");
        }
        if self.episode {
            s.push_str("episode,");
        }
        if self.person {
            s.push_str("person,");
        }
        if self.list {
            s.push_str("list,");
        }

        //s.remove(s.len()-1);

        f.write_str(s.as_str())
    }
}
