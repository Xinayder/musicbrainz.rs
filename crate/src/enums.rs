use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum PersonType {
    Person,
    Group,
    Orchestra,
    Choir,
    Character,
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
