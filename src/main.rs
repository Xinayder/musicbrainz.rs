extern crate musicbrainz;
use musicbrainz::*;

fn main() {
    let app = MusicBrainz::new();
    let artist = app.lookup("4a00ec9d-c635-463a-8cd4-eb61725f0c60").unwrap();
    for album in &artist.albums {
        println!("{:?}", album);
    }
}
