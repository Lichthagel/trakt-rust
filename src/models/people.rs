//! All models related to [people]
//!
//! [people]: https://trakt.docs.apiary.io/#reference/people
use crate::models::{Ids, Movie, Show};

/// A [person]
///
/// [person]: https://trakt.docs.apiary.io/#reference/people
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub ids: Ids,
}

/// [Credits] of a [person]
///
/// [Credits]: https://trakt.docs.apiary.io/#reference/people/movies/get-movie-credits
/// [person]: struct.Person.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct Credits {
    pub cast: Option<Vec<Character>>,
    pub crew: Option<Crew>,
}

/// Crew in [credits]
///
/// [credits]: struct.Credits.html
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

/// Crewmember in crew in [credits]
///
/// [credits]: struct.Credits.html
#[derive(Debug, Serialize, Deserialize)]
pub struct CrewMember {
    pub job: String,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}

/// A character played by someone
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub character: String,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}

/// People/credits of a movie/show
#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    pub cast: Vec<CastPerson>,
    pub crew: Option<CrewPeople>,
}

/// crew in [People]
///
/// [People]: struct.People.html
#[derive(Debug, Serialize, Deserialize)]
pub struct CrewPeople {
    pub art: Option<Vec<CrewPerson>>,
    pub production: Option<Vec<CrewPerson>>,
    pub crew: Option<Vec<CrewPerson>>,
    #[serde(rename = "costume & make-up")]
    pub costume_and_make_up: Option<Vec<CrewPerson>>,
    pub directing: Option<Vec<CrewPerson>>,
    pub writing: Option<Vec<CrewPerson>>,
    pub sound: Option<Vec<CrewPerson>>,
    pub camera: Option<Vec<CrewPerson>>,
}

/// A [person] acting a character
///
/// [person]: struct.Person.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct CastPerson {
    pub character: String,
    pub person: Person,
}

/// A [person] working in the [crew]
///
/// [person]: struct.Person.rs
/// [crew]: struct.CrewPeople.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct CrewPerson {
    pub job: String,
    pub person: Person,
}
