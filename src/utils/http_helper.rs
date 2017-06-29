use std::env;
use std::error::Error;
use std::io::Read;
use std::io;
use std::io::prelude::*;
use hyper;
use hyper::status::{StatusClass, StatusCode};
use hyper::client::response::Response;
use hyper::method::Method;
use hyper::header;
use dotenv;
use rocket;
use rocket::response::{Content, Responder};
use rocket::http::ContentType;
use rocket::data::Data;

fn check_status<'r>(
    response: Response,
    content_type: ContentType,
) -> Result<rocket::Response<'r>, Box<Error>> {
    match response.status.class() {
        StatusClass::Success => {
            match response.status {
                StatusCode::NoContent => {
                    match rocket::response::status::NoContent.respond() {
                        Ok(r) => {
                            let mut final_resp = rocket::Response::build_from(r);
                            final_resp.header(hyper::header::AccessControlAllowOrigin::Any);
                            return final_resp.ok();
                        }
                        Err(_) => {
                            return Err(Box::new(
                                io::Error::new(io::ErrorKind::Other, "Bad Request!"),
                            ))
                        }
                    }
                }
                _ => {
                    let s = rocket::response::Stream::from(response);
                    let cors_response = match Content(content_type, s).respond() {
                        Ok(r) => r,
                        Err(_) => {
                            return Err(Box::new(
                                io::Error::new(io::ErrorKind::Other, "Bad Request!"),
                            ))
                        }
                    };
                    let mut final_resp = rocket::Response::build_from(cors_response);
                    final_resp.header(hyper::header::AccessControlAllowOrigin::Any);
                    final_resp.ok()
                }
            }
        }
        _ => Err(Box::new(
            io::Error::new(io::ErrorKind::Other, "Bad Request!"),
        )),
    }
}

fn post_data<'r>(
    uri: String,
    data: Data,
    content_type: ContentType,
) -> Result<rocket::Response<'r>, Box<Error>> {
    dotenv::dotenv().ok();
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + &uri;
    let url = try!(hyper::Url::parse(&store_uri));
    let mut content = data.open();
    let mut buf = vec![];
    try!(content.read_to_end(&mut buf));
    let mut req = try!(hyper::client::request::Request::new(
        hyper::method::Method::Post,
        url,
    ));
    req.headers_mut().set(hyper::header::ContentLength(
        buf.len() as u64,
    ));
    let mut str_req = try!(req.start());
    try!(str_req.write_all(buf.as_slice()));
    try!(str_req.flush());
    let response = try!(str_req.send());
    check_status(response, content_type)
}

fn qry<'r>(
    uri: String,
    method: Method,
    content_type: ContentType,
) -> Result<rocket::Response<'r>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_STORE_URL")) + &uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let response = try!(client.request(method, url).send());
    check_status(response, content_type)
}

pub fn post_credentials<'r>(uri: String, body: String) -> Result<rocket::Response<'r>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_AUTH")) + &uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let mut headers = header::Headers::new();
    headers.set(header::ContentType::form_url_encoded());
    let response = try!(
        client
            .request(Method::Post, url)
            .headers(headers)
            .body(&body)
            .send()
    );
    check_status(response, ContentType::Plain)
}

pub fn post_verify<'r>(
    uri: String,
    content_type: ContentType,
) -> Result<rocket::Response<'r>, Box<Error>> {
    let store_uri = try!(env::var("SCHANI_AUTH")) + &uri;
    let client = hyper::client::Client::new();
    let url = try!(hyper::Url::parse(&store_uri));
    let response = try!(client.request(Method::Post, url).send());
    check_status(response, content_type)
}

pub fn get_x<'r>(uri: String, content_type: ContentType) -> Option<rocket::Response<'r>> {
    let result = qry(uri, Method::Get, content_type);
    info!("{:?}", result);
    // qry(uri, Method::Get, content_type).ok()
    result.ok()
}

pub fn post_x<'r>(uri: String, content_type: ContentType) -> Option<rocket::Response<'r>> {
    qry(uri, Method::Post, content_type).ok()
}

pub fn put_x<'r>(uri: String, content_type: ContentType) -> Option<rocket::Response<'r>> {
    qry(uri, Method::Put, content_type).ok()
}

pub fn post_file_x<'r>(
    uri: String,
    data: Data,
    content_type: ContentType,
) -> Option<rocket::Response<'r>> {
    post_data(uri, data, content_type).ok()
}
