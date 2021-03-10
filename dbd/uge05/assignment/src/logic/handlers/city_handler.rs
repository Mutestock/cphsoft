use crate::connection::conn::get_pool;
use crate::entities::city::{City, NewCity};
use crate::entities::entity_utils::CRUD;
use warp;

pub async fn create(new_city: NewCity) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in city_handler create");

    let response = City::create(&conn, new_city).await;

    let reply = match response {
        Ok(city) => {
            println!("{:#?}", &city);
            201
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn read(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in city_handler read");

    let response = City::read(&conn, id).await;

    let reply = match response {
        Ok(city) => city,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update( id: i32, new_city: NewCity) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in city_handler read");

    let response = City::update(&conn, new_city, id).await;

    let reply = match response {
        Ok(city) => 204,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in city_handler read");

    let response = City::delete(&conn, id).await;

    let reply = match response {
        Ok(city) => 204,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in city_handler read");

    let response = City::list(&conn).await;

    let reply = match response {
        Ok(citys) => citys,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}
