#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate schani_apigateway;
extern crate rocket;
extern crate hyper;
extern crate dotenv;
extern crate rocket_contrib;

use self::schani_apigateway::routes::{images, collections, tags};

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api",
                           routes![images::get_images,
                                   images::get_image,
                                   images::get_sidecar_file,
                                   images::get_image_file,
                                   images::new_image,
                                   images::new_image_file,
                                   images::new_sidecar_file,
                                   images::update,
                                   collections::get_collections,
                                   collections::get_collection,
                                   collections::new_collection,
                                   collections::update,
                                   tags::get_tags,
                                   tags::get_tag,
                                   tags::new_tag,
                                   tags::update])
}

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}
