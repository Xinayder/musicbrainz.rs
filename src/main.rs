extern crate musicbrainz;
use musicbrainz::*;

fn main() {
    let app = MusicBrainz::new();
    let results = app.search("deadmau5");
    for result in results {
        match app.lookup(result) {
            Some(x) => println!("{:#?}", x),
            None => println!("error")
        };
    }
}
