use uuid::Uuid;
use enums::*;
use std::collections::HashMap;
use std::fmt;
use traits::Entity;
use error::Error;

#[derive(Debug, Clone)]
pub struct ReleaseGroup {
    pub title: String,
    pub release_date: String,
    pub id: Uuid,
    pub artist: Uuid,
    pub primary_type: AlbumType,
    pub secondary_types: Vec<AlbumType>
}

impl ReleaseGroup {
    pub fn new(title: String, release_date: String, id: Uuid, artist: Uuid, primary_type: AlbumType, secondary_types: Vec<AlbumType>) -> ReleaseGroup {
        ReleaseGroup {
            title: title,
            release_date: release_date,
            id: id,
            artist: artist,
            primary_type: primary_type,
            secondary_types: secondary_types
        }
    }
}

impl PartialEq for ReleaseGroup {
    fn eq(&self, other: &ReleaseGroup) -> bool {
        self.id == other.id && self.artist == other.artist
    }
}

impl fmt::Display for ReleaseGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{primary} {name}", primary=self.primary_type, name=self.title);
        writeln!(f, "Id: {id}", id=self.id)
    }
}

impl Entity for ReleaseGroup {
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {
        let data = match client.get("release-group", params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        let count = data["count"].as_i32().unwrap();
        let mut results: Vec<ReleaseGroup> = Vec::new();

        if count == 0 {
            return Ok(results);
        }

        let albums = &data["release-groups"];

        for album in albums.members() {
            if album["score"] == "100" {
                let id = match album["id"].as_str() {
                    Some(x) => {
                        match Uuid::parse_str(x) {
                            Ok(y) => y,
                            Err(e) => return Err(Error::ParseUuid(e))
                        }
                    },
                    None => return Err(Error::AsSlice)
                };

                let secondary_types: Vec<AlbumType> = Vec::new();
                let artist_credits = &album["artist-credit"];
                let artist_id = match artist_credits[0]["artist"]["id"].as_str() {
                    Some(x) => {
                        match Uuid::parse_str(x) {
                            Ok(y) => y,
                            Err(e) => return Err(Error::ParseUuid(e))
                        }
                    },
                    None => return Err(Error::AsSlice)
                };

                let album_type = match album["primary_type"].as_str() {
                    Some(x) => x.parse::<AlbumType>().unwrap(),
                    None => return Err(Error::AsSlice)
                };

                results.push(ReleaseGroup::new(
                    album["title"].to_string(),
                    String::new(),
                    id,
                    artist_id,
                    album_type,
                    secondary_types
                ))
            }
        }
        Ok(results)
    }

    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error> {
        let mut album_data = match client.get(&format!("release-group/{id}", id=entity_id), params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        if !album_data["error"].is_null() {
            let error_msg = album_data["error"].to_string();
            return Err(Error::Http(error_msg));
        }

        let album_type = match album_data["primary-type"].as_str() {
            Some(x) => x.parse::<AlbumType>().unwrap(),
            None => return Err(Error::AsSlice)
        };

        let mut secondary_types: Vec<AlbumType> = Vec::new();
        if !album_data["secondary-types"].is_null() && !album_data["secondary-types"].is_empty() {
            for secondary_type in album_data["secondary-types"].members() {
                secondary_types.push(
                    match secondary_type.as_str() {
                        Some(x) => x.parse::<AlbumType>().unwrap(),
                        None => return Err(Error::AsSlice)
                    }
                );
            }
        }

        let mut artist: Uuid = Uuid::nil();
        if !album_data["artist-credit"].is_null() && !album_data["artist-credit"].is_empty() {
            let artist_credit = album_data["artist-credit"].pop();
            artist = match artist_credit["artist"]["id"].as_str() {
                Some(x) => {
                    match Uuid::parse_str(x) {
                        Ok(y) => y,
                        Err(e) => return Err(Error::ParseUuid(e))
                    }
                },
                None => return Err(Error::AsSlice)
            };
        }

        let album_id = match album_data["id"].as_str() {
            Some(x) => {
                match Uuid::parse_str(x) {
                    Ok(y) => y,
                    Err(e) => return Err(Error::ParseUuid(e))
                }
            },
            None => return Err(Error::AsSlice)
        };

        Ok(ReleaseGroup::new(
            album_data["title"].to_string(),
            album_data["first-release-date"].to_string(),
            album_id,
            artist,
            album_type,
            secondary_types
        ))
    }
}
