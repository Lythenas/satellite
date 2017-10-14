use std::path::{Path, PathBuf};
//use std::collections::HashMap;

use serde::Serialize;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;
use rocket::response::{Redirect, Failure};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Flash;

use context_builder::ContextBuilder;
use db::DbConn;
use db::models::Post;
use controllers::posts::{self, NewPost};
use forms::posts::NewPostForm;
//use response::ResponseResult;
use routes::Urlify;

pub fn routes() -> Vec<Route> {
    routes![index, static_files, new_post_form, new_post, get_post_short, get_post_long, test_flash]
}

pub fn prepare_context_builder<'a, T: Serialize>(current_url: Option<&'a str>, context_builder: &mut ContextBuilder<'a, T>) {
    let menu_builder = context_builder.menu_builder("main");

    if let Some(url) = current_url {
        menu_builder.set_active(url);
    }
}

#[get("/")]
fn index(db: DbConn, mut context_builder: ContextBuilder<Vec<Post>>) -> Template {
    prepare_context_builder(Some("/"), &mut context_builder);

    let posts = posts::posts(&db);

    let context = context_builder.finalize_with_data(posts);

    Template::render("frontend/index", &context)
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

    match posts::try_insert(&db, &post) {
        Ok(post) => {
            // TODO use general url generation in post_get
            Ok(Flash::success(Redirect::to(format!("/post/{}", post.id).as_str()), "Post created successfully."))
        },
        Err(errors) => {
            prepare_context_builder(Some("/post/new"), &mut context_builder);
            let context = context_builder.finalize_with_data(
                NewPostForm::with_errors(post, errors)
            );
            Err(Template::render("frontend/create", &context))
        }
    }
}

#[get("/post/<id>")]
fn get_post_short(id: i32, db: DbConn) -> Result<Redirect, Failure> {
    if id < 0 {
        return Err(Failure(Status::NotFound))
    }
    match posts::get_with_id(&db, id) {
        Ok(post) => {
            Ok(Redirect::to(&post.url()))
        },
        Err(_) => {
            Err(Failure(Status::NotFound))
        }
    }
}

#[get("/post/<slug>", rank = 2)]
fn get_post_long(slug: String, db: DbConn, mut context_builder: ContextBuilder<Post>) -> Result<Template, Failure> {
    match posts::get_with_slug(&db, slug) {
        Ok(post) => {
            prepare_context_builder(Some("/post"), &mut context_builder);
            let context = context_builder.finalize_with_data(post);
            Ok(Template::render("frontend/post", &context))
        },
        Err(_) => {
            Err(Failure(Status::NotFound))
        }
    }
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
