use models::{Tag, NewTag};
use utils::http_helper::{get_x, post_x, put_x};
use rocket::Response;
use rocket::http::ContentType;

#[get("/tags")]
fn get_tags<'r>() -> Option<Response<'r>> {
    let store_uri = "/tags".to_string();
    get_x(store_uri, ContentType::JSON)
}

#[get("/tags/<id>")]
fn get_tag<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/tags/{}", id);
    get_x(store_uri, ContentType::JSON)
}

#[post("/tags/new?<new_tag>")]
fn new_tag<'r>(new_tag: NewTag) -> Option<Response<'r>> {
    let store_uri = format!("/tags/new?label={}", new_tag.label);
    post_x(store_uri, ContentType::JSON)
}

#[put("/tags/update?<tag>")]
fn update<'r>(tag: Tag) -> Option<Response<'r>> {
    let store_uri = format!("/tags/update?id={}label={}", tag.id, tag.label);
    put_x(store_uri, ContentType::JSON)
}
