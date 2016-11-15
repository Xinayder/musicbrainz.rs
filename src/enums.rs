use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
/// The PersonType enum is used to state whether an artist is a person, a group, or something else.
pub enum PersonType {
    /// Indicates an individual person.
    Person,
    /// Indicates a group of people that may or may not have a distinctive name.
    Group,
    /// Indicates an orchestra (a large instrumental ensemble).
    Orchestra,
    /// Indicates a choir/chorus (a large vocal ensemble).
    Choir,
    /// Indicates an individual fictional character.
    Character,
    /// Anything which does not fit into the above categories.
    Other
}

impl FromStr for PersonType {
    type Err = ();

    fn from_str(s: &str) -> Result<PersonType, ()> {
        match s {
            "Person" => Ok(PersonType::Person),
            "Group" => Ok(PersonType::Group),
            "Orchestra" => Ok(PersonType::Orchestra),
            "Choir" => Ok(PersonType::Choir),
            "Character" => Ok(PersonType::Character),
            "Other" => Ok(PersonType::Other),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PersonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PersonType::Person => write!(f, "Person"),
            PersonType::Group => write!(f, "Group"),
            PersonType::Orchestra => write!(f, "Orchestra"),
            PersonType::Choir => write!(f, "Choir"),
            PersonType::Character => write!(f, "Character"),
            PersonType::Other => write!(f, "Other")
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlbumMainType {
    Album,
    Single,
    EP,
    Broadcast,
    Other
}

impl FromStr for AlbumMainType {
    type Err = ();

    fn from_str(s: &str) -> Result<AlbumMainType, ()> {
        match s {
            "Album" => Ok(AlbumMainType::Album),
            "Single" => Ok(AlbumMainType::Single),
            "EP" => Ok(AlbumMainType::EP),
            "Broadcast" => Ok(AlbumMainType::Broadcast),
            "Other" => Ok(AlbumMainType::Other),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlbumSecondaryType {
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    Live,
    Remix,
    DjMix,
    MixtapeStreet
}

impl FromStr for AlbumSecondaryType {
    type Err = ();

    fn from_str(s: &str) -> Result<AlbumSecondaryType, ()> {
        match s {
            "Compilation" => Ok(AlbumSecondaryType::Compilation),
            "Soundtrack" => Ok(AlbumSecondaryType::Soundtrack),
            "Spokenword" => Ok(AlbumSecondaryType::Spokenword),
            "Interview" => Ok(AlbumSecondaryType::Interview),
            "Audiobook" => Ok(AlbumSecondaryType::Audiobook),
            "Live" => Ok(AlbumSecondaryType::Live),
            "Remix" => Ok(AlbumSecondaryType::Remix),
            "DJ-mix" => Ok(AlbumSecondaryType::DjMix),
            "Mixtape/Street" => Ok(AlbumSecondaryType::MixtapeStreet),
            _ => Err(())
        }
    }
}
