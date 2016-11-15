extern crate musicbrainz;

use musicbrainz::*;
use std::collections::HashMap;

fn main() {
    let mb = MusicBrainz::new();
    let mut query = HashMap::new();

    query.insert("query", "metallica");
    let results = mb.search_artist(&mut query);

    println!("{:?}", results[0]);
}