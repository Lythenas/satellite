use chrono::NaiveDateTime;
use routes::Urlify;

#[derive(Debug, Clone, Eq, PartialEq, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author: String,
    pub published_at: Option<NaiveDateTime>,
}

impl Post {
    const BASE_URL: &'static str = "/post";
}

impl Urlify for Post {
    fn url(&self) -> String {
        format!("{}/{}", Self::BASE_URL, self.slug)
    }

    fn short_url(&self) -> String {
        format!("{}/{}", Self::BASE_URL, self.id)
    }
}