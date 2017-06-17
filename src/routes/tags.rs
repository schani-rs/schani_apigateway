use rocket::response::Content;
use models::{Tag, NewTag};
use utils::http_helper::{get_json, post_json, put_json};

#[get("/tags")]
fn get_collections() -> Option<Content<String>> {
    let store_uri = "/collections";
    get_json(&store_uri)
}

#[get("/tags/<id>")]
fn get_collection(id: i32) -> Option<Content<String>> {
    let store_uri = format!("/collections/{}", id);
    get_json(&store_uri)
}

#[post("/tags/new?<new_tag>")]
fn new_collection(new_tag: NewTag) -> Option<Content<String>> {
    let store_uri = format!("/images/label?label={}", new_tag.label);
    post_json(&store_uri)
}

#[put("/tags/update?<tag>")]
fn update(tag: Tag) -> Option<Content<String>> {
    let store_uri = format!("/images/label?id={}label={}", tag.id, tag.label);
    put_json(&store_uri)
}
