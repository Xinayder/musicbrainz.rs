extern crate musicbrainz;

use musicbrainz::*;
use std::collections::HashMap;

fn main() {
    let mb = MusicBrainz::new();
    let mut query = HashMap::new();

    query.insert("query", "metallica");
    let results = mb.artist().search(&mb, &mut query);

    println!("{:?}", results[0]);

    query.clear();
    query.insert("inc", "release-groups+tags");
    let artist_id = musicbrainz::Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
                            .expect("failed to parse artist ID as Uuid");
    let result = mb.artist().lookup(&mb, &artist_id, &mut query);

    println!("{:#?}", result);
}