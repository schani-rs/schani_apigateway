use models::{Collection, NewCollection};
use utils::http_helper::{get_json, post_json, put_json};
use rocket::Response;

#[get("/collections")]
fn get_collections<'r>() -> Option<Response<'r>> {
    let store_uri = "/collections".to_string();
    get_json(store_uri)
}

#[get("/collections/<id>")]
fn get_collection<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/collections/{}", id);
    get_json(store_uri)
}

#[post("/collections/new?<new_collection>")]
fn new_collection<'r>(new_collection: NewCollection) -> Option<Response<'r>> {
    let store_uri = format!("/images/new?name={}&description={}",
                            new_collection.name,
                            new_collection.description);
    post_json(store_uri)
}

#[put("/collections/update?<collection>")]
fn update<'r>(collection: Collection) -> Option<Response<'r>> {
    let store_uri = format!("/images/update?id={}name={}&description={}",
                            collection.id,
                            collection.name,
                            collection.description);
    put_json(store_uri)
}
