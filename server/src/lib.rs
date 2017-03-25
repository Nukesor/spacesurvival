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

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;

mod schema;
mod validation;
mod models;
mod data;
mod responses;
mod api;
mod helpers;
mod handlers;
mod statics;


pub fn rocket_factory() -> rocket::Rocket {
//    let ref values = data::researches::RESEARCH_LIST;
//    match values.get(&data::types::ResearchTypes::PlasmaGenerator) {
//        Some(lol) => println!("{:?}", lol.name),
//        None => println!("{:?}", "Nix gefunden".to_string()),
//    }
//
//    let ref values = data::modules::MODULE_LIST;
//    match values.get(&data::types::ModuleTypes::Turret) {
//        Some(lol) => println!("{:?}", lol.level[0].shoots.as_ref().unwrap().range),
//        None => println!("{:?}", "Nix gefunden".to_string()),
//    }
    rocket::ignite()
        .manage(helpers::db::init_db_pool())
        .mount("/", routes![statics::index])
        .mount("/static", routes![statics::static_files])
        .mount("/api/user/", routes![
               api::user::user::info,
               api::user::user::register,
               api::user::user::settings,
        ])
        .mount("/api/auth/", routes![
               api::auth::auth::login,
        ])
        .mount("/api/pod/", routes![
               api::pod::pod::add_to_queue,
               api::pod::pod::settings,
        ])
        .catch(errors![handlers::bad_request_handler, handlers::unauthorized_handler,
                       handlers::forbidden_handler, handlers::not_found_handler,
                       handlers::internal_server_error_handler,
                       handlers::service_unavailable_handler])
}
