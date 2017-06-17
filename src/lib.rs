#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
extern crate rocket;
extern crate hyper;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

pub mod routes;
pub mod models;
pub mod utils;
