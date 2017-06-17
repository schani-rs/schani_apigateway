use rocket::response::Content;
use models::{Collection, NewCollection};
use utils::http_helper::{get_json, post_json, put_json};

#[get("/collections")]
fn get_collections() -> Option<Content<String>> {
    let store_uri = "/collections";
    get_json(&store_uri)
}

#[get("/collections/<id>")]
fn get_collection(id: i32) -> Option<Content<String>> {
    let store_uri = format!("/collections/{}", id);
    get_json(&store_uri)
}

#[post("/collections/new?<new_collection>")]
fn new_collection(new_collection: NewCollection) -> Option<Content<String>> {
    let store_uri = format!("/images/new?name={}&description={}",
                            new_collection.name,
                            new_collection.description);
    post_json(&store_uri)
}

#[put("/collections/update?<collection>")]
fn update(collection: Collection) -> Option<Content<String>> {
    let store_uri = format!("/images/update?id={}name={}&description={}",
                            collection.id,
                            collection.name,
                            collection.description);
    put_json(&store_uri)
}
