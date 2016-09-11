use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Album {
    pub title: String,
    pub release_date: String,
    pub id: Uuid,
    pub artist: Uuid
}

impl Album {
    pub fn new(title: String, release_date: String, id: Uuid, artist: Uuid) -> Album {
        Album {
            title: title,
            release_date: release_date,
            id: id,
            artist: artist
        }
    }
}
