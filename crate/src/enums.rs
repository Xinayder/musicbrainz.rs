#[derive(Debug, Clone)]
pub enum PersonType {
    Person,
    Group,
    Orchestra,
    Choir,
    Character,
    Other
}

use std::str::FromStr;
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
