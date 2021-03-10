use warp::{self, Filter};
use dotenv;
#[macro_use]
use sqlx;

mod entities;
mod connection;
mod logic;
mod service;

use self::{
    service::{
        routes::{
            caretaker_routes,
            city_routes,
            pet_routes,
            vet_routes,
        }
    },
    logic::{
        handlers::{
            caretaker_handler,
            pet_handler,
            vet_handler,
            city_handler,
        }
    }
};


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    //env::set_var("DATABASE_URL", "postgres://postgres_user:postgres_pass@localhost:13337/postgres_db");
    dotenv::dotenv()
        .ok();

    let pool = connection::conn::get_pool()
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type","Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"])
        .build();

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    //let router = create_user!()
    //    .with(cors);

    println!("Starting server ...");

    warp::serve(hello)
        .run(([0, 0, 0, 0],16969))
        .await;

    Ok(())
}
