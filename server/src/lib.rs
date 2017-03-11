#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate uuid;
extern crate rocket;
extern crate serde_json;
extern crate validator;
extern crate jsonwebtoken;
extern crate chrono;
extern crate argon2rs;
extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

mod schema;
mod validation;
mod models;
mod responses;

pub mod api;
pub mod helpers;
pub mod handlers;
pub mod statics;
