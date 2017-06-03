extern crate spacelib;
use spacelib::rocket_factory;

use spacelib::models::user::User;
use spacelib::update::update::tick;
use spacelib::helpers::db::{DB,init_db_pool};

use std::time::Duration;
use std::thread;

fn main() {
    #[cfg(debug_assertions)]
    create_debug_user();

    thread::spawn(|| {
        spawn_tick_thread();
    });

    rocket_factory().launch();
}

fn spawn_tick_thread() {
    let pool = init_db_pool();
    let db = match pool.get() {
        Ok(conn) => DB::new(conn),
        Err(_) => panic!("Failed to get database transaction"),
    };
    loop {
        tick(&db);
        thread::sleep(Duration::from_millis(1000));
    }
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
