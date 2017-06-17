use rocket::response::Content;
use models::NewImage;
use rocket::data::Data;
use utils::http_helper::{get_json, post_json, post_file};

#[get("/images")]
fn get_images() -> Option<Content<String>> {
    let store_uri = "/images";
    match get_json(&store_uri) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[get("/images/<id>")]
fn get_image(id: i32) -> Option<Content<String>> {
    let store_uri = format!("/images/{}", id);
    match get_json(&store_uri) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[get("/images/<id>/sidecar")]
fn get_sidecar_file(id: i32) -> Option<Content<String>> {
    let store_uri = format!("/images/{}/sidecar", id);
    match get_json(&store_uri) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[get("/images/<id>/file")]
fn get_image_file(id: i32) -> Option<Content<String>> {
    let store_uri = format!("/images/{}/file", id);
    match get_json(&store_uri) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[post("/images/new?<new_image>")]
fn new_image(new_image: NewImage) -> Option<Content<String>> {
    let store_uri = format!("/images/new?title={}&description={}&license={}&side_car_file={}&raw_image_id={}",
                            new_image.title,
                            new_image.description,
                            new_image.license,
                            new_image.side_car_file,
                            new_image.raw_image_id);
    match post_json(&store_uri) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[post("/images/<id>/file/new", data="<data>")]
fn new_image_file(id: i32, data: Data) -> Option<Content<String>> {
    let store_uri = "/images/".to_string() + &id.to_string() + "/file/new";
    match post_file(&store_uri, data) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}

#[post("/images/<id>/sidecar/new", data="<data>")]
fn new_sidecar_file(id: i32, data: Data) -> Option<Content<String>> {
    let store_uri = "/images/".to_string() + &id.to_string() + "/sidecar/new";
    match post_file(&store_uri, data) {
        Ok(response) => Some(response),
        Err(_) => None,
    }
}
