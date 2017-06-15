#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate dotenv;
#[macro_use]
extern crate rocket_contrib;
extern crate core;

use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};
use futures::{Future, Stream};
use hyper::{Client, Response};
use tokio_core::reactor::Core;
use std::env;
use std::error::Error;
use std::str;
use rocket_contrib::JSON;
use core::str::FromStr;
use std::io::Read;

fn fetchGet(uri: &String) -> Result<Response, Box<Error>> {
    let mut core = try!(Core::new());
    let client = Client::new(&core.handle());
    let uri_string = try!(env::var("SCHANI_STORE_URL")) + "/" + uri;
    let hyper_uri = try!(uri_string.parse());
    let work = client.get(hyper_uri);
    let result = try!(core.run(work));
    Ok(result)
}

// #[get("/images")]
// fn get_images() -> Result<JSON<String>, Box<Error>> {
//     let response = try!(fetchGet(&"/images".to_string()));
//     let mut res_string = String::new();
//     let mut res_u8 = Vec::<&u8>::new();
//     response
//         .body()
//         .for_each(|chunk| res_u8.push(chunk.as_ref()));
//     try!(response.read_to_string());
//     Ok(JSON(response))
// }

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files])
}

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}
