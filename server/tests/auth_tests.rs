use rocket;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method,ContentType};

use spacelib::{api, handlers};


describe! auth_tests {
    before_each {
        let rocket = rocket::ignite()
            .mount("/api/auth/", routes![
                   api::auth::auth::register,
                   api::auth::auth::login
            ])
            .catch(errors![handlers::bad_request_handler, handlers::unauthorized_handler,
                handlers::forbidden_handler, handlers::not_found_handler,
                handlers::internal_server_error_handler,
                handlers::service_unavailable_handler]);
    }

    describe! status {
        before_each {
            let mut req = MockRequest::new(Method::Post, "/api/auth/register")
                .header(ContentType::JSON)
                .body(r#"{ "username": "admin", "password": "hunter2"}"#);
            let mut res = req.dispatch_with(&rocket);
            let body_str = res.body().and_then(|b| b.into_string()).unwrap();
        }

        it "responds with status Created 201" {
            assert_eq!(res.status(), Status::Created);
        }
    }
}
