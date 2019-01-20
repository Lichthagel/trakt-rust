use crate::models::Person;
use crate::models::{Movie, Show};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credits {
    cast: Option<Vec<Character>>,
    crew: Option<Crew>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Crew {
    #[serde(rename = "art")]
    Art(Vec<CrewMember>),
    #[serde(rename = "directing")]
    Directing(Vec<CrewMember>),
    #[serde(rename = "editing")]
    Editing(Vec<CrewMember>),
    #[serde(rename = "producing")]
    Producing(Vec<CrewMember>),
    #[serde(rename = "production")]
    Production(Vec<CrewMember>),
    #[serde(rename = "costume & make-up")]
    CostumeAndMakeUp(Vec<CrewMember>),
    #[serde(rename = "writing")]
    Writing(Vec<CrewMember>),
    #[serde(rename = "sound")]
    Sound(Vec<CrewMember>),
    #[serde(rename = "camera")]
    Camera(Vec<CrewMember>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewMember {
    job: String,
    show: Option<Show>,
    movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    character: String,
    show: Option<Show>,
    movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    cast: Vec<CastPerson>,
    crew: Option<CrewPeople>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewPeople {
    art: Option<Vec<CrewPerson>>,
    production: Option<Vec<CrewPerson>>,
    crew: Option<Vec<CrewPerson>>,
    #[serde(rename = "costume & make-up")]
    costume_and_make_up: Option<Vec<CrewPerson>>,
    directing: Option<Vec<CrewPerson>>,
    writing: Option<Vec<CrewPerson>>,
    sound: Option<Vec<CrewPerson>>,
    camera: Option<Vec<CrewPerson>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastPerson {
    character: String,
    person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewPerson {
    job: String,
    person: Person,
}

pub enum FilterType {
    All,
    Personal,
    Official,
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            FilterType::All => "all",
            FilterType::Personal => "personal",
            FilterType::Official => "official",
        })
    }
}

pub enum ListsSorting {
    Popular,
    Likes,
    Comments,
    Items,
    Added,
    Updated,
}

impl fmt::Display for ListsSorting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ListsSorting::Popular => "popular",
            ListsSorting::Likes => "likes",
            ListsSorting::Comments => "comments",
            ListsSorting::Items => "items",
            ListsSorting::Added => "added",
            ListsSorting::Updated => "updated",
        })
    }
}

pub struct PeopleListSearchFactory {
    pub filter_type: FilterType,
    pub sorting: ListsSorting,
}

impl PeopleListSearchFactory {
    pub fn with_filter_type(mut self, filter_type: FilterType) -> PeopleListSearchFactory {
        self.filter_type = filter_type;
        self
    }

    pub fn with_sorting(mut self, sorting: ListsSorting) -> PeopleListSearchFactory {
        self.sorting = sorting;
        self
    }
}

impl Default for PeopleListSearchFactory {
    fn default() -> Self {
        PeopleListSearchFactory {
            filter_type: FilterType::All,
            sorting: ListsSorting::Popular,
        }
    }
}
