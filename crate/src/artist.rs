use json::parse;
use std::io::Read;
use album::Album;
use uuid::Uuid;

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub gender: String,
    pub id: Uuid,
    pub tags: Vec<String>,
    pub albums: Vec<Album>
}

impl Artist {
    pub fn new(name: String, gender: String) -> Artist {
        Artist {
            name: name,
            gender: gender,
            id: Uuid::nil(),
            tags: Vec::new(),
            albums: Vec::new(),
        }
    }
}

pub trait ArtistTrait {
    fn search(self, query: &str) -> Vec<Artist>;
    fn lookup(self, id: &str) -> Option<Artist>;
}

impl ArtistTrait for super::MusicBrainz {
    fn search(self, name: &str) -> Vec<Artist> {
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

                        let mut id = Uuid::parse_str(artist["id"].as_str()
                            .expect("failed to parse artist ID as slice"))
                            .expect("failed to parse artist ID as Uuid");

                        let mut artist_obj = Artist::new(name, gender);
                        artist_obj.id = id;

                        for tag in artist["tags"].members() {
                            artist_obj.tags.push(tag["name"].to_string());
                        }

                        results.push(artist_obj);
                    }
                }
            }
        }
        results
    }

    fn lookup(self, id: &str) -> Option<Artist> {
        let endpoint = format!("https://musicbrainz.org/ws/2/artist/{id}?inc=release-groups&fmt=json", id=id);
        let mut res = self.get(&endpoint).expect("failed to lookup artist");

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response to string");

        let data = parse(&buf).unwrap();
        let mut result = Artist::new(data["name"].to_string(), data["gender"].to_string());
        result.id = Uuid::parse_str(data["id"].as_str()
            .expect("failed to parse artist ID as slice"))
            .expect("failed to parse artist ID as Uuid");

        let albums = &data["release-groups"];
        for album in albums.members() {
            result.albums.push(Album {
                title: album["title"].to_string(),
                release_date: album["first-release-date"].to_string(),
                id: Uuid::parse_str(album["id"].as_str()
                    .expect("failed to parse release group ID as slice"))
                    .expect("failed to parse release group ID as Uuid"),
                artist: result.id
            });
        }

        Some(result)
    }
}
