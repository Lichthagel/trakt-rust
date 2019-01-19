use crate::models::Movie;

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
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    character: String,
    movie: Movie,
}
