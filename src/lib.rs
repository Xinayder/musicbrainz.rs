extern crate hyper;
extern crate json;
extern crate uuid;
extern crate url;

use std::collections::HashMap;
use std::io::Read;
use url::{Url};

#[derive(Debug)]
pub struct MusicBrainz {
    client: hyper::Client,
    user_agent: String
}

impl MusicBrainz {
    /// Instantiates a new `MusicBrainz` struct.
    ///
    /// The `MusicBrainz` struct contains useful methods required by the library.
    /// It must be instantiated before using the implemented methods.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// let musicbrainz = MusicBrainz::new();
    /// ```
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

    fn get(&self, url: &str, params: &HashMap<&str, &str>) -> json::Result<json::JsonValue> {
        let base_uri = "https://musicbrainz.org/ws/2";
        let mut endpoint = Url::parse(&format!("{}/{}", base_uri, url))
            .expect("error parsing URL");

        endpoint.query_pairs_mut().append_pair("fmt", "json");
        for (param, val) in params {
            endpoint.query_pairs_mut().append_pair(param, val);
        }

        let user_agent = self.user_agent.clone();
        let mut res = self.client.get(endpoint)
            .header(hyper::header::UserAgent(user_agent))
            .send()
            .expect(&format!("failed to get url '{}'", url));

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response body to string");

        json::parse(&buf)
    }

    pub fn artist(&self) -> artist::Artist {
        artist::Artist::empty()
    }

}

pub mod artist;
pub mod release_group;
pub mod enums;
pub mod traits;

pub use traits::*;
pub use uuid::Uuid;