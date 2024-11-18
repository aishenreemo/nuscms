#[macro_use] extern crate rocket;

use rocket::tokio::sync::OnceCell;
use rocket::serde::json::Json;
use sqlx::MySqlPool;

static DB_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket!"
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    DB_POOL.set(pool).unwrap();

    rocket::build().mount("/", routes![index])
}
