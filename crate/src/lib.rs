extern crate hyper;
extern crate json;
extern crate uuid;

use std::io::Read;

#[derive(Debug)]
pub struct MusicBrainz {
    client: hyper::Client,
    user_agent: String
}

impl MusicBrainz {
    pub fn new() -> MusicBrainz {
        let user_agent = format!("{name}/{version} ( {homepage} )",
            name=env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION"),
            homepage=env!("CARGO_PKG_HOMEPAGE")
        );

        MusicBrainz {
            client: hyper::Client::new(),
            user_agent: user_agent
        }
    }

    pub fn get(&self, url: &str) -> json::Result<json::JsonValue> {
        let user_agent = self.user_agent.clone();
        let mut res = self.client.get(url).header(hyper::header::UserAgent(user_agent))
            .send()
            .expect(&format!("failed to get url '{}'", url));

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response body to string");

        json::parse(&buf)
    }

}

pub mod artist;
pub mod album;
pub mod enums;

pub use artist::ArtistTrait;
