use crate::models::Person;
use crate::models::{Movie, Show};

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
