use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::Serialize;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;
use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::response::Flash;

use context_builder::{ContextBuilder};
use db::models::Post;
use db::schema::posts;
use db::DbConn;
use diesel;

pub fn routes() -> Vec<Route> {
    routes![index, static_files, new_post_form, new_post, test_flash]
}

pub fn prepare_context_builder<'a, T: Serialize>(current_url: Option<&'a str>, context_builder: &mut ContextBuilder<'a, T>) {
    let menu_builder = context_builder.menu_builder("main");

    if let Some(url) = current_url {
        menu_builder.set_active(url);
    }
}

fn posts(db: &DbConn) -> Vec<Post> {
    use diesel::prelude::*;
    use db::models::*;
    use db::schema::posts::dsl::*;

    posts.filter(published_at.is_not_null())
        .order(published_at.desc())
        .limit(5)
        .load::<Post>(&**db)
        .expect("error loading posts")
}

#[get("/")]
fn index(db: DbConn, mut context_builder: ContextBuilder<Vec<Post>>) -> Template {
    prepare_context_builder(Some("/"), &mut context_builder);

    let posts = posts(&db);

    let context = context_builder.finalize_with_data(posts);

    Template::render("frontend/index", &context)
}

#[derive(Debug, Clone)]
struct NonEmpty(String);

impl<'v> FromFormValue<'v> for NonEmpty {
    type Error = &'static str;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        if form_value.is_empty() {
            Err("can't be empty")
        } else {
            match form_value.url_decode() {
                Ok(s) => Ok(NonEmpty(s)),
                Err(_) => Err("could not be decoded as utf-8")
            }
        }
    }
}

#[derive(Debug, Clone, FromForm)]
struct NewPost {
    title: Result<NonEmpty, &'static str>,
    author: Result<NonEmpty, &'static str>,
    body: Result<NonEmpty, &'static str>,
}

impl NewPost {
    fn errors(&self) -> HashMap<String, String> {
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
struct NewDbPost {
    title: String,
    author: String,
    body: String,
}

impl<'a, 'r> From<&'a NewPost> for Result<NewDbPost, HashMap<String, String>> {
    fn from(post: &'a NewPost) -> Self {
        let errors = post.errors();
        if errors.is_empty() {
            let post = post.clone();
            // safe to unwrap
            Ok(NewDbPost {
                title: post.title.unwrap().0,
                author: post.author.unwrap().0,
                body: post.body.unwrap().0,
            })
        } else {
            Err(errors)
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize)]
struct NewPostForm {
    errors: HashMap<String, String>,
    title: String,
    author: String,
    body: String,
}

impl NewPostForm {
    fn with_errors(post: NewPost, errors: HashMap<String, String>) -> NewPostForm {
        fn unwrap<T>(r: Result<NonEmpty, T>) -> String {
            match r {
                Ok(s) => s.0,
                Err(_) => String::new(),
            }
        }

        NewPostForm {
            errors,
            title: unwrap(post.title),
            author: unwrap(post.author),
            body: unwrap(post.body),
        }
    }
}

#[get("/post")]
fn new_post_form(mut context_builder: ContextBuilder<NewPostForm>) -> Template {
    prepare_context_builder(Some("/post/new"), &mut context_builder);

    let context = context_builder.finalize_with_default();

    Template::render("frontend/create", &context)
}

#[post("/post", data = "<post>")]
fn new_post<'a>(db: DbConn, post: Form<'a, NewPost>, mut context_builder: ContextBuilder<NewPostForm>) -> Result<Flash<Redirect>, Template> {
    let post = post.into_inner();

    let errors = match (&post).into() {
        Ok(post) => if insert_post(&db, &post) {
            None
        } else {
            let mut m = HashMap::new();
            m.insert("general".to_string(), "Error saving your post. Please try again later.".to_string());
            Some(m)
        },
        Err(e) => Some(e),
    };

    if let Some(errors) = errors {
        prepare_context_builder(Some("/post/new"), &mut context_builder);
        let context = context_builder.finalize_with_data(
            NewPostForm::with_errors(post, errors)
        );
        Err(Template::render("frontend/create", &context))
    } else {
        Ok(Flash::success(Redirect::to("/"), "Post created successfully."))
    }

}

fn insert_post(db: &DbConn, post: &NewDbPost) -> bool {
    use db::schema::posts;
    use diesel::ExecuteDsl;

    diesel::insert(post).into(posts::table)
        .execute(&**db)
        .map(|num_inserted| num_inserted > 0)
        .unwrap_or(false)
}

#[get("/test-flash/<name>/<msg>")]
fn test_flash(name: String, msg: String) -> Flash<Redirect> {
    Flash::new(Redirect::to("/"), name, msg)
}

// TODO add more routes

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
