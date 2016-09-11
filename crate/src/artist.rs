use json::parse;
use std::io::Read;
use album::Album;
use uuid::Uuid;
use enums::{PersonType};

#[derive(Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub gender: String,
    pub id: Uuid,
    pub tags: Vec<String>,
    pub albums: Vec<Album>,
    pub artist_type: PersonType
}

impl Artist {
    pub fn new(name: String, gender: String, id: Uuid, tags: Vec<String>, albums: Vec<Album>, artist_type: PersonType) -> Artist {
        Artist {
            name: name,
            gender: gender,
            id: id,
            tags: tags,
            albums: albums,
            artist_type: artist_type
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
        let mut res = self.get(&endpoint).expect("failed to search for artist");

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response to string");

        let data = parse(&buf).unwrap();
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

                        results.push(Artist::new(name, gender, id, tags, albums, PersonType::Other));
                    }
                }
            }
        }
        results
    }

    fn lookup(&self, artist: Artist) -> Option<Artist> {
        let id = artist.id.hyphenated().to_string();
        let endpoint = format!("https://musicbrainz.org/ws/2/artist/{id}?inc=release-groups&fmt=json", id=&id);
        let mut res = self.get(&endpoint).expect("failed to lookup artist");

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response to string");

        let artist_data = parse(&buf).unwrap();
        let artist = artist.clone();
        let artist_type = artist_data["type"].as_str().expect("failed to parse artist type to slice").parse::<PersonType>().unwrap();
        let mut artist_albums: Vec<Album> = Vec::new();

        let albums = &artist_data["release-groups"];
        for album in albums.members() {
            artist_albums.push(Album {
                title: album["title"].to_string(),
                release_date: album["first-release-date"].to_string(),
                id: Uuid::parse_str(album["id"].as_str()
                    .expect("failed to parse release group ID as slice"))
                    .expect("failed to parse release group ID as Uuid"),
                artist: artist.id
            });
        }

        Some(Artist::new(artist.name, artist.gender, artist.id, artist.tags, artist_albums, artist_type))
    }
}
