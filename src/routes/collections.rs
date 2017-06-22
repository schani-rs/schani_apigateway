use models::{Collection, NewCollection};
use utils::http_helper::{get_x, post_x, put_x};
use rocket::Response;
use rocket::http::ContentType;

#[get("/collections")]
fn get_collections<'r>() -> Option<Response<'r>> {
    let store_uri = "/collections".to_string();
    get_x(store_uri, ContentType::JSON)
}

#[get("/collections/<id>")]
fn get_collection<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/collections/{}", id);
    get_x(store_uri, ContentType::JSON)
}

#[post("/collections/new?<new_collection>")]
fn new_collection<'r>(new_collection: NewCollection) -> Option<Response<'r>> {
    let store_uri = format!(
        "/images/new?name={}&description={}",
        new_collection.name,
        new_collection.description
    );
    post_x(store_uri, ContentType::JSON)
}

#[put("/collections/update?<collection>")]
fn update<'r>(collection: Collection) -> Option<Response<'r>> {
    let store_uri = format!(
        "/images/update?id={}name={}&description={}",
        collection.id,
        collection.name,
        collection.description
    );
    put_x(store_uri, ContentType::JSON)
}

#[post("/images/<image_id>/collections/<collection_id>")]
fn new_image_collection<'r>(image_id: i32, collection_id: i32) -> Option<Response<'r>> {
    let store_uri =
        format!("/images/{}/collections/{}",
        image_id,
        collection_id,
    );
    post_x(store_uri, ContentType::JSON)
}
