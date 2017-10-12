use chrono::NaiveDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String,
    pub published_at: Option<NaiveDateTime>,
}
