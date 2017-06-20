use diesel;
use diesel::prelude::*;

use helpers::db::{DB, init_db_pool};

use models::module::NewModule;
use models::user::User;

use schema::modules;


#[allow(dead_code)]
pub fn create_debug_user() {
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
    for mut resource in resources {
        resource.update_resource(5000, false, &db)
    }

    // Create the new module
    let new_module = NewModule {
        name: "PlasmaGenerator".to_string(),
        stationary: false,
        level: 1,
        x_pos: Some(2),
        y_pos: Some(2),

        pod_id: Some(pod.id),
        base_id: None,
    };

    diesel::insert(&new_module)
        .into(modules::table)
        .execute(&*db)
        .expect("Failed to create module.");
    pod.update_resource_production(&db);
}
