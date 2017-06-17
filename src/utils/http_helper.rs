use std::env;
use std::error::Error;
use std::io::Read;
use std::io::prelude::*;
use hyper;
use dotenv;
use rocket::response::Content;
use rocket::http::ContentType;
use rocket::data::Data;

pub fn get_json(uri: &str) -> Result<Content<String>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let mut response = try!(client.request(hyper::method::Method::Get, url).send());
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    Ok(Content(ContentType::JSON, body))
}

pub fn post_json(uri: &str) -> Result<Content<String>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let mut response = try!(client.request(hyper::method::Method::Post, url).send());
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    Ok(Content(ContentType::JSON, body))
}

pub fn post_file(uri: &str, data: Data) -> Result<Content<String>, Box<Error>> {
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
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    Ok(Content(ContentType::JSON, body))
}
