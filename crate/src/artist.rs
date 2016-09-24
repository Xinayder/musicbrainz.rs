use release_group::ReleaseGroup;
use uuid::Uuid;
use enums::{PersonType, AlbumMainType, AlbumSecondaryType};

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

/// Provides methods for browsing, looking up or searching artists.
pub trait ArtistTrait {
    fn search(&self, query: &str) -> Vec<Artist>;
    fn lookup(&self, artist: Artist) -> Option<Artist>;
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
    /// let musicbrainz = MusicBrainz::new();
    /// let search_results = musicbrainz.search("deadmau5");
    ///
    /// assert_eq!(search_results[0].id.hyphenated().to_string(), "4a00ec9d-c635-463a-8cd4-eb61725f0c60");
    /// ```
    fn search(&self, query: &str) -> Vec<Artist> {
        let data = self.get(&format!("artist?query={}&fmt=json", query)).unwrap();

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
    fn lookup(&self, artist: Artist) -> Option<Artist> {
        let artist = artist.clone();
        let id = artist.id.hyphenated().to_string();

        let artist_data = self.get(&format!("artist/{id}?inc=release-groups&fmt=json", id=&id)).unwrap();
        let artist_type = artist_data["type"].as_str().expect("failed to parse artist type as slice").parse::<PersonType>().unwrap();
        let mut artist_albums: Vec<ReleaseGroup> = Vec::new();

        let release_groups = &artist_data["release-groups"];
        for album in release_groups.members() {
            let mut secondary_types: Vec<AlbumSecondaryType> = Vec::new();
            for secondary_type in album["secondary-types"].members() {
                secondary_types.push(secondary_type.as_str()
                    .expect("failed to parse album secondary type as slice")
                    .parse::<AlbumSecondaryType>()
                    .unwrap())
            }

            artist_albums.push(ReleaseGroup {
                title: album["title"].to_string(),
                release_date: album["first-release-date"].to_string(),
                id: Uuid::parse_str(album["id"].as_str()
                    .expect("failed to parse release group ID as slice"))
                    .expect("failed to parse release group ID as Uuid"),
                artist: artist.id,
                primary_type: album["primary-type"].as_str()
                    .expect("failed to parse album primary type as slice")
                    .parse::<AlbumMainType>()
                    .unwrap(),
                secondary_types: secondary_types
            });
        }

        Some(Artist::new(artist.id, artist.name, artist.gender, artist_type, artist.tags, artist_albums))
    }
}
