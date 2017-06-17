#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate schani_apigateway;
extern crate rocket;
extern crate hyper;
extern crate dotenv;
extern crate rocket_contrib;

use self::schani_apigateway::routes::images;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api",
                           routes![images::get_images,
                                   images::get_image,
                                   images::get_sidecar_file,
                                   images::get_image_file,
                                   images::new_image,
                                   images::new_image_file,
                                   images::new_sidecar_file])
}

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}
