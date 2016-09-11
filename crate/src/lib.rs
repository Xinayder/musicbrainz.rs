extern crate hyper;
extern crate json;

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

    pub fn get(self, url: &str) -> hyper::error::Result<hyper::client::response::Response> {
        self.client.get(url).header(hyper::header::UserAgent(self.user_agent)).send()
    }
}

pub mod artist;
pub mod album;

pub use artist::ArtistTrait;
