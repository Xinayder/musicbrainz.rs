use uuid::Uuid;

#[derive(Debug)]
pub struct Album {
    pub title: String,
    pub release_date: String,
    pub id: Uuid,
    pub artist: Uuid
}

impl Album {
    pub fn new(title: String, id: Uuid) -> Album {
        Album {
            title: title,
            release_date: String::new(),
            id: Uuid::nil(),
            artist: Uuid::nil()
        }
    }
}
