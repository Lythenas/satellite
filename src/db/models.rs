use chrono::NaiveDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String,
    pub published_at: Option<NaiveDateTime>,
}

impl Post {
    /// Creates a url-slug from the title.
    /// Lowercases the title and replaces all whitespace with dashes ("-").
    pub fn slug(&self) -> String {
        self.title.to_lowercase().replace(|c: char| c.is_whitespace(), "-")
            .replace("#", "").replace("/", "")
    }
}