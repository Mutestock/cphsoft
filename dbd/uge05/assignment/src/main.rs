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

    let city_routes = create_city!()
        .or(read_city!())
        .or(update_city!())
        .or(delete_city!())
        .or(read_list_city!());

    let caretaker_routes = create_caretaker!()
        .or(read_caretaker!())
        .or(update_caretaker!())
        .or(delete_caretaker!())
        .or(read_list_caretaker!());

    let pet_routes = create_pet!()
        .or(read_pet!())
        .or(update_pet!())
        .or(delete_pet!())
        .or(read_list_pet!());
    
    let vet_routes = create_vet!()
        .or(read_vet!())
        .or(update_vet!())
        .or(delete_vet!())
        .or(read_list_vet!());
    
    let router = city_routes
        .or(caretaker_routes)
        .or(pet_routes)
        .or(vet_routes)
        .with(cors);

    warp::serve(router)
        .run(([0, 0, 0, 0],16969))
        .await;


    Ok(())
}
