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
pub enum AlbumType {
    Album,
    Single,
    EP,
    Broadcast,
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    Live,
    Remix,
    DjMix,
    MixtapeStreet,
    Other,
}

impl FromStr for AlbumType {
    type Err = ();

    fn from_str(s: &str) -> Result<AlbumType, ()> {
        match s {
            "Album" => Ok(AlbumType::Album),
            "Single" => Ok(AlbumType::Single),
            "EP" => Ok(AlbumType::EP),
            "Broadcast" => Ok(AlbumType::Broadcast),
            "Compilation" => Ok(AlbumType::Compilation),
            "Soundtrack" => Ok(AlbumType::Soundtrack),
            "Spokenword" => Ok(AlbumType::Spokenword),
            "Interview" => Ok(AlbumType::Interview),
            "Audiobook" => Ok(AlbumType::Audiobook),
            "Live" => Ok(AlbumType::Live),
            "Remix" => Ok(AlbumType::Remix),
            "DJ-mix" => Ok(AlbumType::DjMix),
            "Mixtape/Street" => Ok(AlbumType::MixtapeStreet),
            "Other" => Ok(AlbumType::Other),
            _ => Err(())
        }
    }
}

impl fmt::Display for AlbumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlbumType::Album => write!(f, "Album"),
            AlbumType::Single => write!(f, "Single"),
            AlbumType::EP => write!(f, "EP"),
            AlbumType::Broadcast => write!(f, "Broadcast"),
            AlbumType::Compilation => write!(f, "Compilation"),
            AlbumType::Soundtrack => write!(f, "Soundtrack"),
            AlbumType::Spokenword => write!(f, "Spokenword"),
            AlbumType::Interview => write!(f, "Interview"),
            AlbumType::Audiobook => write!(f, "Audiobook"),
            AlbumType::Live => write!(f, "Live"),
            AlbumType::Remix => write!(f, "Remix"),
            AlbumType::DjMix => write!(f, "DJ-mix"),
            AlbumType::MixtapeStreet => write!(f, "Mixtape/Street"),
            AlbumType::Other => write!(f, "Other")
        }
    }
}