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
use response::ResponseResult;
use request::IdSlug;

pub fn routes() -> Vec<Route> {
    routes![index, static_files, new_post_form, new_post, get_post, test_flash]
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

#[get("/post/<id_slug>")]
fn get_post(id_slug: IdSlug, db: DbConn, mut context_builder: ContextBuilder<Post>) -> ResponseResult<Template> {
    let id = match id_slug.id.ok_or(Failure(Status::NotFound)) {
        Ok(id) => id,
        Err(err) => return err.into(),
    };
    let slug = id_slug.slug;
    match posts::get(&db, id) {
        Ok(post) => {
            {
                let real_slug = &post.slug;
                if slug.is_none() || slug.unwrap() != *real_slug {
                    // TODO move url generation somewhere else (maybe model or controller)
                    return ResponseResult::Forward(Redirect::to(format!("/post/{}-{}", id, real_slug).as_str()))
                }
            }

            prepare_context_builder(Some("/post"), &mut context_builder);
            let context = context_builder.finalize_with_data(post);
            ResponseResult::Success(Template::render("frontend/post", &context))
        },
        Err(_) => {
            ResponseResult::Failure(Failure(Status::NotFound))
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
