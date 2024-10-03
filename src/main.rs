use rocket::{get, launch, routes};
use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::MySqlPool;
use dotenv::dotenv;

#[derive(Database)]
#[database("my_db")]
struct MyDb(MySqlPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();

    rocket::build()
        .attach(MyDb::init())
        .mount("/", routes![index])
}
