#[derive(Debug)]
pub struct Album {
    pub title: String,
    pub release_date: String,
    pub id: String,
    pub artist: String
}

impl Album {
    pub fn new(title: String, id: String) -> Album {
        Album {
            title: title,
            release_date: String::new(),
            id: String::new(),
            artist: String::new()
        }
    }
}
