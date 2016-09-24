extern crate musicbrainz;
use musicbrainz::*;

use std::collections::HashMap;

fn main() {
    let app = MusicBrainz::new();

    let mut query = HashMap::new();
    query.insert("query", "metallica");

    let results = app.search(&mut query);
    for result in results {
        query.clear();
        query.insert("inc", "release-groups+tags");
        match app.lookup(result, &mut query) {
            Some(x) => println!("{:#?}", x),
            None => println!("error")
        };
    }
}
