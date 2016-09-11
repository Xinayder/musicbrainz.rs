use json::parse;
use std::io::Read;

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub gender: String,
    pub id: String,
    pub tags: Vec<String>,
}

impl Artist {
    pub fn new(name: String, gender: String) -> Artist {
        Artist {
            name: name,
            gender: gender,
            id: String::new(),
            tags: Vec::new(),
            albums: Vec::new(),
        }
    }
}

pub trait ArtistTrait {
    fn search(self, query: &str) -> Option<Vec<Artist>>;
    fn lookup(self, id: &str) -> Option<Artist>;
}

impl ArtistTrait for super::MusicBrainz {
    fn search(self, name: &str) -> Option<Vec<Artist>> {
        let endpoint = format!("https://musicbrainz.org/ws/2/artist?query={}&fmt=json", name);
        let mut res = self.get(&endpoint).expect("failed to search for artist");

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response to string");

        let data = parse(&buf).unwrap();
        let count = data["count"].as_i32().unwrap();

        if count == 0 {
            return None;
        }

        let artists = &data["artists"];
        let mut results: Vec<Artist> = Vec::new();

        for artist in artists.members() {
            if !artist["score"].is_null() {
                if artist["score"] == "100" {
                    if !artist["name"].is_null() {
                        let name = artist["name"].to_string();
                        let gender = artist["gender"].to_string();
                        let id = artist["id"].to_string();

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
        Some(results)
    }
}
