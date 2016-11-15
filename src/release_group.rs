use uuid::Uuid;
use enums::*;
use std::collections::HashMap;
use traits::Entity;

#[derive(Debug, Clone)]
pub struct ReleaseGroup {
    pub title: String,
    pub release_date: String,
    pub id: Uuid,
    pub artist: Uuid,
    pub primary_type: AlbumMainType,
    pub secondary_types: Vec<AlbumSecondaryType>
}

impl ReleaseGroup {
    pub fn new(title: String, release_date: String, id: Uuid, artist: Uuid, primary_type: AlbumMainType, secondary_types: Vec<AlbumSecondaryType>) -> ReleaseGroup {
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

impl Entity for ReleaseGroup {
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Vec<Self> {
        let data = client.get("release-group", params).unwrap();

        let count = data["count"].as_i32().unwrap();

        if count == 0 {
            return Vec::new();
        }

        let albums = &data["release-groups"];
        let mut results: Vec<ReleaseGroup> = Vec::new();

        for album in albums.members() {
            if album["score"] == "100" {
                let id = Uuid::parse_str(album["id"].as_str()
                    .expect("failed to parse artist ID as slice"))
                    .expect("failed to parse artist ID as Uuid");
                let secondary_types: Vec<AlbumSecondaryType> = Vec::new();
                let artist_credits = &album["artist-credit"];

                results.push(ReleaseGroup::new(
                    album["title"].to_string(),
                    String::new(),
                    id,
                    Uuid::parse_str(artist_credits[0]["artist"]["id"].as_str()
                        .expect("failed to parse artist ID as slice"))
                        .expect("failed to parse artist ID as Uuid"),
                    album["primary-type"].as_str()
                        .expect("failed to parse album primary type as slice")
                        .parse::<AlbumMainType>()
                        .unwrap(),
                    secondary_types
                ))
            }
        }
        results
    }

    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, String> {
        let mut album_data = client.get(&format!("release-group/{id}", id=entity_id), params).unwrap();

        if !album_data["error"].is_null() {
            let error_msg = album_data["error"].to_string();
            return Err(format!("error looking up release group: {msg}", msg=error_msg));
        }

        let album_type = album_data["primary-type"].as_str()
            .expect("failed to parse release group type as slice")
            .parse::<AlbumMainType>()
            .unwrap();

        let mut secondary_types: Vec<AlbumSecondaryType> = Vec::new();
        if !album_data["secondary-types"].is_null() && !album_data["secondary-types"].is_empty() {
            for secondary_type in album_data["secondary-types"].members() {
                secondary_types.push(
                        secondary_type.as_str()
                        .expect("failed to parse release group secondary type as slice")
                        .parse::<AlbumSecondaryType>()
                        .unwrap()
                );
            }
        }

        let mut artist: Uuid = Uuid::nil();
        if !album_data["artist-credit"].is_null() && !album_data["artist-credit"].is_empty() {
            let artist_credit = album_data["artist-credit"].pop();
            artist = Uuid::parse_str(artist_credit["artist"]["id"].as_str()
                        .expect("failed to parse artist credit ID as slice"))
                        .expect("failed to parse artist credit ID as Uuid");
        }

        Ok(ReleaseGroup::new(
            album_data["title"].to_string(),
            album_data["first-release-date"].to_string(),
            Uuid::parse_str(album_data["id"].as_str()
                .expect("failed to parse release group ID as slice"))
                .expect("failed to parse release group ID as Uuid"),
            artist,
            album_type,
            secondary_types
        ))
    }
}
