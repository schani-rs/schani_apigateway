use models::{Image, NewImage};
use rocket::data::Data;
use utils::http_helper::{get_x, post_x, put_x, post_file_x};
use rocket::Response;
use rocket::http::ContentType;

#[get("/images")]
fn get_images<'r>() -> Option<Response<'r>> {
    let store_uri = "/images".to_string();
    get_x(store_uri, ContentType::JSON)
}

#[get("/images/<id>")]
fn get_image<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/images/{}", id);
    get_x(store_uri, ContentType::JSON)
}

#[get("/images/<id>/tags")]
fn get_tags_of_image<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/images/{}/tags", id);
    get_x(store_uri, ContentType::JSON)
}

#[get("/images/<id>/sidecar")]
fn get_sidecar_file<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/images/{}/sidecar", id);
    get_x(store_uri, ContentType::Plain)
}

#[get("/images/<id>/file")]
fn get_image_file<'r>(id: i32) -> Option<Response<'r>> {
    let store_uri = format!("/images/{}/file", id);
    get_x(store_uri, ContentType::JPEG)
}

#[post("/images/new?<new_image>")]
fn new_image<'r>(new_image: NewImage) -> Option<Response<'r>> {
    let store_uri = format!("/images/new?title={}&description={}&license={}&side_car_file={}&raw_image_id={}",
                            new_image.title,
                            new_image.description,
                            new_image.license,
                            new_image.side_car_file,
                            new_image.raw_image_id);
    post_x(store_uri, ContentType::JSON)
}

#[post("/images/<id>/file/new", data="<data>")]
fn new_image_file<'r>(id: i32, data: Data) -> Option<Response<'r>> {
    let store_uri = "/images/".to_string() + &id.to_string() + "/file/new";
    post_file_x(store_uri, data, ContentType::JSON)
}

#[post("/images/<id>/sidecar/new", data="<data>")]
fn new_sidecar_file<'r>(id: i32, data: Data) -> Option<Response<'r>> {
    let store_uri = "/images/".to_string() + &id.to_string() + "/sidecar/new";
    post_file_x(store_uri, data, ContentType::JSON)
}

#[put("/images/update?<image>")]
fn update<'r>(image: Image) -> Option<Response<'r>> {
    let store_uri = format!("/images/update?id={}title={}&description={}&license={}&side_car_file={}&raw_image_id={}",
                            image.id,
                            image.title,
                            image.description,
                            image.license,
                            image.side_car_file,
                            image.raw_image_id);
    put_x(store_uri, ContentType::JSON)
}
