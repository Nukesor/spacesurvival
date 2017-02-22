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

mod api;
mod validation;
mod models;
mod schema;
mod handlers;
mod statics;
mod responses;
mod helpers;


fn main() {
    rocket::ignite()
        .manage(helpers::db::init_db_pool())
        .mount("/", routes![statics::index])
        .mount("/static", routes![statics::static_files])
        .mount("/api/hello/", routes![api::hello::whoami])
        .mount("/api/auth/", routes![
               api::auth::login,
               api::auth::register,
        ])
        .catch(errors![handlers::bad_request_handler, handlers::unauthorized_handler,
                       handlers::forbidden_handler, handlers::not_found_handler,
                       handlers::internal_server_error_handler,
                       handlers::service_unavailable_handler])
        .launch();
}
