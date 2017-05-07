#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate uuid;
extern crate chrono;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate argon2rs;
extern crate rustc_serialize;
extern crate serde_json;
extern crate serde_yaml;
extern crate validator;
extern crate jsonwebtoken;
extern crate petgraph;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;

mod schema;
mod validation;
mod models;
mod data;
mod responses;
mod api;
mod helpers;
mod handlers;
mod statics;

use data::researches::build_graph;


pub fn rocket_factory() -> rocket::Rocket {
    // Read the research list from yml and check for cycles.
    let ref research_list = data::researches::RESEARCH_LIST;
    build_graph(research_list);

    rocket::ignite()
        .manage(helpers::db::init_db_pool())
        .mount("/", routes![statics::index])
        .mount("/static", routes![statics::static_files])
        .mount("/api/user/",
               routes![
               api::user::user::info,
               api::user::user::register,
               api::user::user::settings,
        ])
        .mount("/api/auth/",
               routes![
               api::auth::auth::login,
        ])
        .mount("/api/pod/",
               routes![
               api::pod::pod::add_module_to_queue,
               api::pod::pod::add_research_to_queue,
               api::pod::pod::settings,
        ])
        .catch(errors![handlers::bad_request_handler,
                       handlers::unauthorized_handler,
                       handlers::forbidden_handler,
                       handlers::not_found_handler,
                       handlers::internal_server_error_handler,
                       handlers::service_unavailable_handler])
}
