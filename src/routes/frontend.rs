use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::Serialize;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;
use rocket::response::Redirect;
use rocket::request::Form;
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


#[derive(Debug, Clone, Eq, PartialEq, Serialize, FromForm, Insertable)]
#[table_name="posts"]
struct NewPost {
    title: String,
    author: String,
    body: String,
}

impl NewPost {
    fn validate(&self) -> HashMap<String, String> {
        let mut errors = HashMap::new();
        if self.title.is_empty() {
            errors.insert("title".into(), "Title can't be empty.".into());
        }
        if self.author.is_empty() {
            errors.insert("author".into(), "Author can't be empty.".into());
        }
        if self.body.is_empty() {
            errors.insert("body".into(), "Body can't be empty.".into());
        }
        errors
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize)]
struct NewPostForm {
    errors: HashMap<String, String>,
    title: String,
    author: String,
    body: String,
}

#[get("/post")]
fn new_post_form(mut context_builder: ContextBuilder<NewPostForm>) -> Template {
    prepare_context_builder(Some("/post/new"), &mut context_builder);

    let context = context_builder.finalize_with_default();

    Template::render("frontend/create", &context)
}

#[post("/post", data = "<post>")]
fn new_post(db: DbConn, post: Form<NewPost>, mut context_builder: ContextBuilder<NewPostForm>) -> Result<Flash<Redirect>, Template> {
    // TODO validate more like rocket intends (see: https://github.com/SergioBenitez/Rocket/blob/master/examples/form_validation/src/main.rs)
    // TODO redo error display

    let post = post.into_inner();
    let errors = post.validate();
    if !errors.is_empty() {
        prepare_context_builder(Some("/post/new"), &mut context_builder);
        let context = context_builder.finalize_with_data(NewPostForm {
            errors,
            title: post.title,
            author: post.author,
            body: post.body,
        });
        Err(Template::render("frontend/create", &context))
    } else if !insert_post(&db, &post) {
        prepare_context_builder(Some("/post/new"), &mut context_builder);
        let context = context_builder.finalize_with_data(NewPostForm {
            errors: {
                let mut m = HashMap::new();
                m.insert("general".into(), "Error saving your post. Please try again later.".into());
                m
            },
            title: post.title,
            author: post.author,
            body: post.body,
        });
        Err(Template::render("frontend/create", &context))
    } else {
        Ok(Flash::success(Redirect::to("/"), "Post created successfully."))
    }

}

fn insert_post(db: &DbConn, post: &NewPost) -> bool {
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
