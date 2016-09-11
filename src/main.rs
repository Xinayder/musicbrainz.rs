extern crate musicbrainz;
use musicbrainz::*;

fn main() {
    let app = MusicBrainz::new();
    let results = app.search("deadmau5");
    for result in results {
        let artist = app.lookup(result);
        println!("{:?}", artist);
    }
}
