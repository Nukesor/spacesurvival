#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate spacelib;

use spacelib::{api, handlers, statics, helpers};

fn main() {
    rocket::ignite()
        .manage(helpers::db::init_db_pool())
        .mount("/", routes![statics::index])
        .mount("/static", routes![statics::static_files])
        .mount("/api/user/", routes![api::user::user::info])
        .mount("/api/auth/", routes![
               api::auth::auth::login,
               api::auth::auth::register,
        ])
        .catch(errors![handlers::bad_request_handler, handlers::unauthorized_handler,
                       handlers::forbidden_handler, handlers::not_found_handler,
                       handlers::internal_server_error_handler,
                       handlers::service_unavailable_handler])
        .launch();
}
