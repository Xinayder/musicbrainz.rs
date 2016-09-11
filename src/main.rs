extern crate musicbrainz;
use musicbrainz::*;

fn main() {
    let app = MusicBrainz::new();
    let data = app.search("florence");
    println!("{:?}", data.unwrap());
}
