#[macro_use] extern crate rocket;
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, MySqlPool};
use dotenv::dotenv;

#[derive(Database)]
#[database("my_db")]
struct MyDb(MySqlPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(MyDb::init())
        .attach(AdHoc::config::<MyDb>())
        .mount("/", routes![index])
}
