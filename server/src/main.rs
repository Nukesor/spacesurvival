extern crate spacelib;

use std::thread;
use std::time::Duration;

use spacelib::rocket_factory;
use spacelib::update::update::tick;
use spacelib::helpers::db::{DB, init_db_pool};
use spacelib::helpers::debug::create_debug_user;


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
