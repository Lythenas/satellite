use std::convert::From;

use rocket::response::{Responder, Failure, Redirect};
use rocket::Response;
use rocket::Request;
use rocket::http::Status;

/// This is a convenience wrapper around the type `T`, `Failure` and `Redirect`.
/// Use this if you need to return all three types from your route.
///
/// ```
/// #[get("/post/<id_slug>")]
/// fn get_post(id_slug: String) -> ResponseResult<Post> {
///     let
///     match Post::get_from_slug(slug) {
///         Ok()
///     }
/// }
/// ``
pub enum ResponseResult<T> {
    Success(T),
    Failure(Failure),
    Forward(Redirect),
}

// TODO move somewhere else
impl<'r, T: Responder<'r>> Responder<'r> for ResponseResult<T> {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        match self {
            ResponseResult::Success(resp) => resp.respond_to(request),
            ResponseResult::Failure(failure) => failure.respond_to(request),
            ResponseResult::Forward(redirect) => redirect.respond_to(request),
        }
    }
}

impl<T> From<Failure> for ResponseResult<T> {
    fn from(failure: Failure) -> Self {
        ResponseResult::Failure(failure)
    }
}

impl<T> From<Redirect> for ResponseResult<T> {
    fn from(redirect: Redirect) -> Self {
        ResponseResult::Forward(redirect)
    }
}