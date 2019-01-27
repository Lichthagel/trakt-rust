use crate::models::Person;
use crate::models::{Movie, Show};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credits {
    pub cast: Option<Vec<Character>>,
    pub crew: Option<Crew>,
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
    pub job: String,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub character: String,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    pub cast: Vec<CastPerson>,
    pub crew: Option<CrewPeople>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CastPerson {
    pub character: String,
    pub person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewPerson {
    pub job: String,
    pub person: Person,
}
