use std::collections::HashMap;
use controllers::posts::NewPost;
use super::NonEmpty;

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize)]
pub struct NewPostForm {
    errors: HashMap<String, String>,
    title: String,
    author: String,
    body: String,
}

impl NewPostForm {
    pub fn with_errors(post: NewPost, errors: HashMap<String, String>) -> NewPostForm {
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
