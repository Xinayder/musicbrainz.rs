use uuid::Uuid;
use enums::*;
use std::collections::HashMap;
use traits::AlbumTrait;

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

impl AlbumTrait for super::MusicBrainz {
    /// Searches MusicBrainz for release groups based on the search query.
    ///
    /// Returns a `Vec` containing the release groups matching the search query.
    /// If no release groups were found, returns an empty `Vec`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// # use std::collections::HashMap;
    /// let musicbrainz = MusicBrainz::new();
    /// let mut query = HashMap::new();
    ///
    /// query.insert("query", "metallica");
    /// let search_results = musicbrainz.search_album(&mut query);
    ///
    /// assert_eq!(search_results[0].id.hyphenated().to_string(), "e8f70201-8899-3f0c-9e07-5d6495bc8046");
    /// ```
    fn search_album(&self, params: &mut HashMap<&str, &str>) -> Vec<ReleaseGroup> {
        params.insert("fmt", "json");
        let data = self.get("release-group", params).unwrap();

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
}
