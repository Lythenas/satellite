use rocket::request::FromParam;
use rocket::http::RawStr;
use std::str::Utf8Error;

// TODO probably remove

/// Request param in the form of `id-slug` where id is `i32` and slug is a url endcoded `String`.
pub struct IdSlug {
    pub id: Option<i32>,
    pub slug: Option<String>,
}

impl<'a> FromParam<'a> for IdSlug {
    type Error = Utf8Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let param = param.url_decode()?;
        let mut split = param.splitn(2, '-');
        let id = split.next().and_then(|id| id.parse::<i32>().ok());
        let slug = split.next().map(String::from);
        Ok(IdSlug { id, slug })
    }
}