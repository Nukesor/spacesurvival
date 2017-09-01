#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(test, plugin(stainless))]
#![recursion_limit="256"]

extern crate uuid;
extern crate chrono;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate argon2rs;
extern crate serde_json;
extern crate serde_yaml;
extern crate validator;
extern crate petgraph;
extern crate ring;

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

pub mod api;
pub mod validation;
pub mod data;
pub mod handlers;
pub mod helpers;
pub mod models;
pub mod update;
pub mod responses;
pub mod schema;
pub mod statics;

#[cfg(test)]
mod tests;

use rocket::fairing::AdHoc;
use chrono::Duration;

use data::modules::get_module_list;
use data::researches::build_research_graph;

pub struct RuntimeConfig(Duration);


pub fn rocket_factory() -> rocket::Rocket {
    // Check if we have valid yml and check research graph for cycles.
    get_module_list();
    build_research_graph();

    rocket::ignite()
        .manage(helpers::db::init_db_pool())
        .attach(AdHoc::on_attach(|rocket| {
            let auth_timeout = rocket
                .config()
                .get_int("auth_token_timeout_days")
                .unwrap_or(7);
            let auth_token_duration = Duration::days(auth_timeout);
            Ok(rocket.manage(RuntimeConfig(auth_token_duration)))
        }))
        .mount("/", routes![statics::index])
        .mount("/static", routes![statics::static_files])
        .mount(
            "/api/auth",
            routes![
               api::auth::auth::login,
        ],
        )
        .mount(
            "/api/user",
            routes![
               api::user::user::info,
               api::user::user::register,
               api::user::user::settings,
        ],
        )
        .mount(
            "/api/pod",
            routes![
               api::pod::pod::settings,
        ],
        )
        .mount(
            "/api/queue",
            routes![
               api::queue::pod::pod_queue_entries,
        ],
        )
        .mount(
            "/api/resources",
            routes![
               api::resources::pod::pod_resources,
        ],
        )
        .mount(
            "/api/researches",
            routes![
               api::research::pod::get_researches,
               api::research::pod::start_research,
               api::research::pod::stop_research,
        ],
        )
        .mount(
            "/api/modules",
            routes![
               api::module::general::get_info,
               api::module::pod::get_modules,
               api::module::pod::add_module,
               api::module::pod::remove_module,
               api::module::pod::upgrade_module,
               api::module::pod::stop_module_upgrade,
        ],
        )
        .catch(errors![
            handlers::bad_request_handler,
            handlers::unauthorized_handler,
            handlers::forbidden_handler,
            handlers::not_found_handler,
            handlers::internal_server_error_handler,
            handlers::service_unavailable_handler,
        ])
}
