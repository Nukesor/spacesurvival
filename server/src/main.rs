extern crate spacelib;
use spacelib::rocket_factory;

use spacelib::models::user::User;
use spacelib::helpers::db::{DB,init_db_pool};

fn main() {
    #[cfg(debug_assertions)]
    create_debug_user();

    rocket_factory().launch();
}


fn create_debug_user() {
    let pool = init_db_pool();
    let db = match pool.get() {
        Ok(conn) => DB::new(conn),
        Err(_) => panic!("Failed to get database transaction"),
    };
    let new_password_hash = User::make_password_hash("hunter2");
    let user = User::new_user("admin".to_string(),
                              "admin@hunter2.de".to_string(),
                              new_password_hash,
                              &db);

    let pod = user.get_pod(&db);

    let resources = pod.get_resources(&db);
    for resource in resources {
        resource.update_resource(5000, false, &db)
    }
}
