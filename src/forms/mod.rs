pub mod posts;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug, Clone)]
pub struct NonEmpty(String);

impl NonEmpty {
    pub fn into_inner(self) -> String {
        self.0
    }
}

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

