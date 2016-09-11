extern crate musicbrainz;



fn main() {
    match musicbrainz::artist::get_by_name("florence and") {
        Some(x) => println!("{:?}", x),
        None => println!("No results found")
    };
}
