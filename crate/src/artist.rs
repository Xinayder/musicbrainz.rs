use album::Album;
use uuid::Uuid;
use enums::{PersonType, AlbumMainType, AlbumSecondaryType};

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub gender: String,
    pub artist_type: PersonType,
    pub tags: Vec<String>,
    pub albums: Vec<Album>
}

impl Artist {
    pub fn new(id: Uuid, name: String, gender: String, artist_type: PersonType, tags: Vec<String>, albums: Vec<Album>) -> Artist {
        Artist {
            id: id,
            name: name,
            gender: gender,
            artist_type: artist_type,
            tags: tags,
            albums: albums
        }
    }
}

pub trait ArtistTrait {
    fn search(&self, query: &str) -> Vec<Artist>;
    fn lookup(&self, artist: Artist) -> Option<Artist>;
}

impl ArtistTrait for super::MusicBrainz {
    fn search(&self, name: &str) -> Vec<Artist> {
        let endpoint = format!("https://musicbrainz.org/ws/2/artist?query={}&fmt=json", name);
        let data = self.get(&endpoint).unwrap();

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
                        let albums: Vec<Album> = Vec::new();

                        for tag in artist["tags"].members() {
                            tags.push(tag["name"].to_string());
                        }

                        results.push(Artist::new(id, name, gender, PersonType::Other, tags, albums));
                    }
                }
            }
        }
        results
    }

    fn lookup(&self, artist: Artist) -> Option<Artist> {
        let artist = artist.clone();
        let id = artist.id.hyphenated().to_string();
        let endpoint = format!("https://musicbrainz.org/ws/2/artist/{id}?inc=release-groups&fmt=json", id=&id);

        let artist_data = self.get(&endpoint).unwrap();
        let artist_type = artist_data["type"].as_str().expect("failed to parse artist type as slice").parse::<PersonType>().unwrap();
        let mut artist_albums: Vec<Album> = Vec::new();

        let albums = &artist_data["release-groups"];
        for album in albums.members() {
            let mut secondary_types: Vec<AlbumSecondaryType> = Vec::new();
            for secondary_type in album["secondary-types"].members() {
                secondary_types.push(secondary_type.as_str()
                    .expect("failed to parse album secondary type as slice")
                    .parse::<AlbumSecondaryType>()
                    .unwrap())
            }

            artist_albums.push(Album {
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
