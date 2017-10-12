use std::collections::HashMap;
use diesel;
use diesel::prelude::*;

use db::models::Post;
use db::schema::posts;
use db::DbConn;
use forms::NonEmpty;

pub fn posts(db: &DbConn) -> Vec<Post> {
    use diesel::prelude::*;
    use db::schema::posts::dsl::*;

    posts.filter(published_at.is_not_null())
        .order(published_at.desc())
        .limit(5)
        .load::<Post>(&**db)
        .expect("error loading posts")
}

pub fn get(db: &DbConn, post_id: i32) -> QueryResult<Post> {
    use diesel::prelude::*;
    use db::schema::posts::dsl::*;

    posts.filter(published_at.is_not_null())
        .filter(id.eq(post_id))
        .first(&**db)
}

pub fn try_insert(db: &DbConn, post: &NewPost) -> Result<Post, HashMap<String, String>> {
    let post: Result<NewDbPost, HashMap<String, String>> = post.into();

    post.and_then(|post| {
        insert_post(&db, &post).map_err(|err| {
            let mut m = HashMap::new();

            // TODO add real logging here
            println!("Error inserting post: {:?}", err);
            m.insert("general".to_string(), "Error saving your post. Please try again later.".to_string());

            m
        })
    })
}

fn insert_post(db: &DbConn, post: &NewDbPost) -> Result<Post, String> {
    diesel::insert(post).into(posts::table)
        .execute(&**db) // TODO use get_result with non sqlite database
        .map_err(|err| format!("{:?}", err))
        .and_then(|num_inserted| {
            if num_inserted < 1 {
                Err("nothing inserted".to_string())
            } else {
                Ok(())
            }
        })
        .and_then(|_| {
            use diesel::prelude::*;
            use db::schema::posts::dsl::*;

            posts.order(id.desc()).first(&**db)
                .map_err(|err| format!("{:?}", err))
        })
}

#[derive(Debug, Clone, FromForm)]
pub struct NewPost {
    pub title: Result<NonEmpty, &'static str>, // TODO add unique constraint, etc.
    pub author: Result<NonEmpty, &'static str>,
    pub body: Result<NonEmpty, &'static str>,
}

impl NewPost {
    pub fn errors(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();

        if let Err(e) = self.title {
            m.insert("title".to_string(), format!("Title {}.", e));
        }
        if let Err(e) = self.author {
            m.insert("author".to_string(), format!("Author {}.", e));
        }
        if let Err(e) = self.body {
            m.insert("body".to_string(), format!("Body {}.", e));
        }

        m
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Insertable)]
#[table_name="posts"]
pub struct NewDbPost {
    pub title: String,
    pub author: String,
    pub body: String,
}

impl<'a, 'r> From<&'a NewPost> for Result<NewDbPost, HashMap<String, String>> {
    fn from(post: &'a NewPost) -> Self {
        let errors = post.errors();
        if errors.is_empty() {
            let post = post.clone();
            // safe to unwrap
            Ok(NewDbPost {
                title: post.title.unwrap().into_inner(),
                author: post.author.unwrap().into_inner(),
                body: post.body.unwrap().into_inner(),
            })
        } else {
            Err(errors)
        }
    }
}
