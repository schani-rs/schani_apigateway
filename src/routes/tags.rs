use models::{Tag, NewTag};
use utils::http_helper::{get_json, post_json, put_json};
use rocket::Response;

#[get("/tags")]
fn get_collections<'r>() -> Option<Response<'r>> {
    let store_uri = "/collections".to_string();
    get_json(store_uri)
}

#[get("/tags/<id>")]
fn get_collection<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/collections/{}", id);
    get_json(store_uri)
}

#[post("/tags/new?<new_tag>")]
fn new_collection<'r>(new_tag: NewTag) -> Option<Response<'r>> {
    let store_uri = format!("/images/label?label={}", new_tag.label);
    post_json(store_uri)
}

#[put("/tags/update?<tag>")]
fn update<'r>(tag: Tag) -> Option<Response<'r>> {
    let store_uri = format!("/images/label?id={}label={}", tag.id, tag.label);
    put_json(store_uri)
}
