use utils::http_helper::{post_verify, post_credentials};
use rocket::Response;
use rocket::http::ContentType;
use rocket::request::Form;

#[derive(Debug, FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/authenticate", data = "<credentials>")]
fn authenticate<'r>(credentials: Form<Credentials>) -> Option<Response<'r>> {
    let auth_uri = "/authenticate".to_string();
    post_credentials(auth_uri, credentials.raw_form_string().to_string()).ok()
}

#[post("/verify/<token>")]
fn verify<'r>(token: &str) -> Option<Response<'r>> {
    let auth_uri = "/verify/".to_string() + token;
    post_verify(auth_uri, ContentType::JSON).ok()
}
