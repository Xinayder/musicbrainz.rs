use release_group::ReleaseGroup;
use uuid::Uuid;
use enums::{PersonType, AlbumMainType, AlbumSecondaryType};
use std::collections::HashMap;
use traits::ArtistTrait;

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
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.id == other.id &&
        self.tags == other.tags &&
        self.name == other.name &&
        self.gender == other.gender &&
        self.release_groups == other.release_groups
    }

    fn ne(&self, other: &Artist) -> bool {
        self.id != other.id
    }
}

impl ArtistTrait for super::MusicBrainz {
    /// Searches MusicBrainz for artists based on the search query.
    ///
    /// Returns a `Vec` containing the artists matching the search query.
    /// If no artists were found, returns an empty `Vec`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// # use std::collections::HashMap;
    /// let musicbrainz = MusicBrainz::new();
    /// let mut query = HashMap::new();
    ///
    /// query.insert("query", "deadmau5");
    ///
    /// let search_results = musicbrainz.search_artist(&mut query);
    ///
    /// assert_eq!(search_results[0].id.hyphenated().to_string(), "4a00ec9d-c635-463a-8cd4-eb61725f0c60");
    /// ```
    fn search_artist(&self, params: &mut HashMap<&str, &str>) -> Vec<Artist> {
        let data = self.get("artist", params).unwrap();

        let count = data["count"].as_i32().unwrap();

        if count == 0 {
            return Vec::new();
        }

        let artists = &data["artists"];
        let mut results: Vec<Artist> = Vec::new();

        for artist in artists.members() {
            if !artist["score"].is_null() {
                if artist["score"] == "100" {
                    if !artist["name"].is_null() {
                        let name = artist["name"].to_string();
                        let gender = artist["gender"].to_string();
                        let id = Uuid::parse_str(artist["id"].as_str()
                            .expect("failed to parse artist ID as slice"))
                            .expect("failed to parse artist ID as Uuid");
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
        results
    }

    /// Lookup an artist by using its MusicBrainz Identifier.
    ///
    fn lookup_artist(&self, artist_id: Uuid, params: &mut HashMap<&str, &str>) -> Result<Artist, String> {
        let artist_data = self.get(&format!("artist/{id}", id=&artist_id), params).unwrap();

        if !artist_data["error"].is_null() {
            let error_msg = artist_data["error"].to_string();
            return Err(format!("error looking up artist: {msg}", msg=error_msg));
        }

        let artist_type = artist_data["type"].as_str()
            .expect("failed to parse artist type as slice")
            .parse::<PersonType>()
            .unwrap();

        let mut tags: Vec<String> = Vec::new();
        if !artist_data["tags"].is_null() {
            for tag in artist_data["tags"].members() {
                tags.push(tag["name"].to_string());
            }
        }

        let mut artist_albums: Vec<ReleaseGroup> = Vec::new();
        if !artist_data["release-groups"].is_null() {
            for album in artist_data["release-groups"].members() {
                let mut secondary_types: Vec<AlbumSecondaryType> = Vec::new();
                for secondary_type in album["secondary-types"].members() {
                    secondary_types.push(secondary_type.as_str()
                        .expect("failed to parse album secondary type as slice")
                        .parse::<AlbumSecondaryType>()
                        .unwrap())
                }

                artist_albums.push(ReleaseGroup::new(
                    album["title"].to_string(),
                    album["first-release-date"].to_string(),
                    Uuid::parse_str(album["id"].as_str()
                        .expect("failed to parse release group ID as slice"))
                        .expect("failed to parse release group ID as Uuid"),
                    Uuid::parse_str(&artist_data["id"].as_str()
                        .expect("failed to parse artist ID as slice"))
                        .expect("failed to parse artist ID as Uuid"),
                    album["primary-type"].as_str()
                        .expect("failed to parse album primary type as slice")
                        .parse::<AlbumMainType>()
                        .unwrap(),
                    secondary_types
                ));
            }
        }

        Ok(Artist::new(
            Uuid::parse_str(artist_data["id"].as_str()
                    .expect("failed to parse artist ID as slice"))
                    .expect("failed to parse artist ID as Uuid"),
            artist_data["name"].to_string(),
            artist_data["gender"].to_string(),
            artist_type,
            tags,
            artist_albums
        ))
    }
}
