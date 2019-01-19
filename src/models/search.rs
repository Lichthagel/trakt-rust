use crate::models::{Episode, List, Movie, Person, SearchItemType, Show};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "type")]
    item_type: SearchItemType,
    score: f64,
    movie: Option<Movie>,
    show: Option<Show>,
    episode: Option<Episode>,
    person: Option<Person>,
    list: Option<List>,
}

pub struct SearchType {
    movie: bool,
    show: bool,
    episode: bool,
    person: bool,
    list: bool,
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
