use dotenv;
use warp::{self, Filter};
#[macro_use]
use sqlx;

mod connection;
mod entities;
mod logic;
mod misc;
mod service;

use self::{
    logic::handlers::{caretaker_handler, city_handler, misc_handler, pet_handler, vet_handler},
    misc::sql_interactions::{
        execute_database_population_script, execute_restricted_user_creation,
    },
    service::routes::{caretaker_routes, city_routes, misc_routes, pet_routes, vet_routes}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //env::set_var("DATABASE_URL", "postgres://postgres_user:postgres_pass@localhost:13337/postgres_db");
    dotenv::dotenv().ok();

    let pool = connection::conn::get_pool().await?;

    sqlx::migrate!().run(&pool).await?;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "content-type",
            "Access-Control-Allow-Origin",
        ])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"])
        .build();

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

    let misc_routes = user_swap!();

    let router = city_routes
        .or(caretaker_routes)
        .or(pet_routes)
        .or(vet_routes)
        .or(misc_routes)
        .with(cors);

    println!("Populating database with entries if non-existant ...");

    execute_database_population_script()
        .await
        .expect("Could not execute the database population script");
    
    execute_restricted_user_creation() 
            .await
            .expect("Could not execute restricted user creation script");

    println!("Starting server ...");

    warp::serve(router).run(([0, 0, 0, 0], 16969)).await;

    Ok(())
}
