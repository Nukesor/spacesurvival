use rocket;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method,ContentType};

use spacelib::rocket_factory;


describe! auth_tests {
    before_each {
        let rocket = rocket_factory();
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
