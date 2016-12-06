use release_group::ReleaseGroup;
use uuid::Uuid;
use enums::{PersonType, AlbumType};
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub gender: String,
    pub artist_type: PersonType,
    pub tags: Vec<String>,
    pub release_groups: Vec<ReleaseGroup>
}

impl Artist {
    pub fn new(id: Uuid, name: String, gender: String, artist_type: PersonType, tags: Vec<String>, release_groups: Vec<ReleaseGroup>) -> Artist {
        Artist {
            id: id,
            name: name,
            gender: gender,
            artist_type: artist_type,
            tags: tags,
            release_groups: release_groups
        }
    }

    pub fn empty() -> Artist {
        Artist::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            PersonType::Other,
            Vec::new(),
            Vec::new()
        )
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.id == other.id &&
        self.name == other.name
    }

    fn ne(&self, other: &Artist) -> bool {
        self.id != other.id
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{name} ({type})", name=self.name, type=self.artist_type);
        writeln!(f, "Id: {id}", id=self.id.hyphenated().to_string())
    }
}

impl Entity for Artist {
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {
        let data = match client.get("artist", params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        let count = data["count"].as_i32().unwrap();
        let mut results : Vec<Artist> = Vec::new();

        if count == 0 {
            return Ok(results);
        }

        let artists = &data["artists"];

        for artist in artists.members() {
            if !artist["score"].is_null() {
                if artist["score"] == "100" {
                    if !artist["name"].is_null() {
                        let name = artist["name"].to_string();
                        let gender = artist["gender"].to_string();

                        let id = match artist["id"].as_str() {
                            Some(x) => {
                                match Uuid::parse_str(x) {
                                    Ok(y) => y,
                                    Err(e) => return Err(Error::ParseUuid(e))
                                }
                            },
                            None => return Err(Error::AsSlice)
                        };

                        let mut tags: Vec<String> = Vec::new();
                        let release_groups: Vec<ReleaseGroup> = Vec::new();

                        for tag in artist["tags"].members() {
                            tags.push(tag["name"].to_string());
                        }

                        results.push(Artist::new(id, name, gender, PersonType::Other, tags, release_groups));
                    }
                }
            }
        }
        Ok(results)
    }

    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error> {
        let artist_data = match client.get(&format!("artist/{id}", id=entity_id), params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        if !artist_data["error"].is_null() {
            let error_msg = artist_data["error"].to_string();
            return Err(Error::Http(error_msg));
        }

        let artist_type = match artist_data["type"].as_str() {
            Some(x) => x.parse::<PersonType>().unwrap(),
            None => return Err(Error::AsSlice)
        };

        let mut tags: Vec<String> = Vec::new();
        if !artist_data["tags"].is_null() {
            for tag in artist_data["tags"].members() {
                tags.push(tag["name"].to_string());
            }
        }

        let artist_id = match artist_data["id"].as_str() {
            Some(x) => {
                match Uuid::parse_str(x) {
                    Ok(y) => y,
                    Err(e) => return Err(Error::ParseUuid(e))
                }
            },
            None => return Err(Error::AsSlice)
        };

        let mut artist_albums: Vec<ReleaseGroup> = Vec::new();
        if !artist_data["release-groups"].is_null() {
            for album in artist_data["release-groups"].members() {
                let mut secondary_types: Vec<AlbumType> = Vec::new();
                for secondary_type in album["secondary-types"].members() {
                    secondary_types.push(match secondary_type.as_str() {
                        Some(x) => x.parse::<AlbumType>().unwrap(),
                        None => return Err(Error::AsSlice)
                    });
                }

                let album_id = match album["id"].as_str() {
                    Some(x) => {
                        match Uuid::parse_str(x) {
                            Ok(y) => y,
                            Err(e) => return Err(Error::ParseUuid(e))
                        }
                    },
                    None => return Err(Error::AsSlice)
                };

                let album_type = match album["primary-type"].as_str() {
                    Some(x) => x.parse::<AlbumType>().unwrap(),
                    None => return Err(Error::AsSlice)
                };

                artist_albums.push(ReleaseGroup::new(
                    album["title"].to_string(),
                    album["first-release-date"].to_string(),
                    album_id,
                    artist_id,
                    album_type,
                    secondary_types
                ));
            }
        }

        Ok(Artist::new(
            artist_id,
            artist_data["name"].to_string(),
            artist_data["gender"].to_string(),
            artist_type,
            tags,
            artist_albums
        ))
    }
}