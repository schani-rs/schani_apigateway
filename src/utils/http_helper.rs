use std::env;
use std::error::Error;
use std::io::Read;
use std::io;
use std::io::prelude::*;
use hyper;
use hyper::status::StatusClass;
use hyper::client::response::Response;
use hyper::method::Method;
use dotenv;
use rocket::response::Content;
use rocket::http::ContentType;
use rocket::data::Data;

fn check_status(response: &mut Response) -> Result<Content<String>, Box<Error>> {
    let mut body = String::new();
    match response.status.class() {
        StatusClass::Success => {
            try!(response.read_to_string(&mut body));
            Ok(Content(ContentType::JSON, body.clone()))
        }
        _ => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Bad Request!"))),
    }
}

fn post_data(uri: &str, data: Data) -> Result<Content<String>, Box<Error>> {
    dotenv::dotenv().ok();
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + uri;
    let url = try!(hyper::Url::parse(&store_uri));
    let mut content = data.open();
    let mut buf = vec![];
    try!(content.read_to_end(&mut buf));
    let mut req = try!(hyper::client::request::Request::new(hyper::method::Method::Post, url));
    req.headers_mut()
        .set(hyper::header::ContentLength(buf.len() as u64));
    let mut str_req = try!(req.start());
    try!(str_req.write_all(buf.as_slice()));
    try!(str_req.flush());
    let mut response = try!(str_req.send());
    check_status(&mut response)
}

fn qry_json(uri: &str, method: Method) -> Result<Content<String>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let mut response = try!(client.request(method, url).send());
    check_status(&mut response)
}

pub fn get_json(uri: &str) -> Option<Content<String>> {
    qry_json(uri, Method::Get).ok()
}

pub fn post_json(uri: &str) -> Option<Content<String>> {
    qry_json(uri, Method::Post).ok()
}

pub fn put_json(uri: &str) -> Option<Content<String>> {
    qry_json(uri, Method::Put).ok()
}

pub fn post_file(uri: &str, data: Data) -> Option<Content<String>> {
    post_data(uri, data).ok()
}
