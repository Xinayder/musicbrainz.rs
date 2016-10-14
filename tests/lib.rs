extern crate musicbrainz;
extern crate uuid;

use uuid::Uuid;
use std::collections::HashMap;
use musicbrainz::*;

#[test]
fn test_search_artist() {
    let m = MusicBrainz::new();
    let generated = artist::Artist::new(
        Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
            .expect("failed to parse artist ID as Uuid"),
        String::from("deadmau5"),
        String::from("male"),
        enums::PersonType::Other,
        vec![
            String::from("electronic"), String::from("house"),
            String::from("seen live"), String::from("electro"),
            String::from("progressive house"), String::from("dance and electronica")
        ],
        Vec::new()
    );

    let mut query = HashMap::new();
    query.insert("query", "deadmau5");

    let correct = m.search_artist(&mut query).remove(0);

    assert_eq!(correct, generated);
}

#[test]
fn test_search_album() {
    let m = MusicBrainz::new();
    let generated = release_group::ReleaseGroup::new(
        String::from("For Lack of a Better Name"),
        String::new(),
        Uuid::parse_str("638a007a-8423-40d6-a97c-f819d320d33c")
            .expect("failed to parse release group ID as Uuid"),
        Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
            .expect("failed to parse artist ID as Uuid"),
        enums::AlbumMainType::Album,
        Vec::new()
    );

    let mut query = HashMap::new();
    query.insert("query", "for lack of a better name");

    let correct = m.search_album(&mut query).remove(0);

    assert_eq!(correct, generated);
}

#[test]
fn test_lookup_artist() {
    let m = MusicBrainz::new();
    let generated = artist::Artist::new(
        Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
            .expect("failed to parse artist ID as Uuid"),
        String::from("deadmau5"),
        String::from("Male"),
        enums::PersonType::Person,
        Vec::new(),
        Vec::new()
    );

    let mut query = HashMap::new();
    let correct = m.lookup_artist(generated.id, &mut query).unwrap();

    assert_eq!(correct, generated);
}

#[test]
#[should_panic]
fn test_lookup_artist_fail() {
    let m = MusicBrainz::new();
    let generated = artist::Artist::new(
        Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
            .expect("failed to parse artist ID as Uuid"),
        String::from("deadmau5"),
        String::from("Male"),
        enums::PersonType::Person,
        Vec::new(),
        Vec::new()
    );

    let mut query = HashMap::new();
    query.insert("inc", "release-groups");
    let correct = m.lookup_artist(generated.id, &mut query).unwrap();

    assert_eq!(correct, generated);
}

#[test]
fn test_lookup_album() {
    let m = MusicBrainz::new();
    let generated = release_group::ReleaseGroup::new(
        String::from("For Lack of a Better Name"),
        String::new(),
        Uuid::parse_str("638a007a-8423-40d6-a97c-f819d320d33c")
            .expect("failed to parse release group ID as Uuid"),
        Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60")
            .expect("failed to parse artist ID as Uuid"),
        enums::AlbumMainType::Album,
        Vec::new()
    );

    let mut query = HashMap::new();
    query.insert("inc", "artist-credits");
    let correct = m.lookup_album(generated.id, &mut query).unwrap();

    assert_eq!(correct, generated);
}
