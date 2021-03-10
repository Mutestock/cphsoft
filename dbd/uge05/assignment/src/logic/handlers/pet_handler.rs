use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use crate::entities::pet::{NewPet, Pet};
use warp;

pub async fn create(new_pet: NewPet) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in pet_handler create");

    let response = Pet::create(&conn, new_pet).await;

    let reply = match response {
        Ok(pet) => {
            println!("{:#?}", &pet);
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
        .expect("Pool could not be created in pet_handler read");

    let response = Pet::read(&conn, id).await;

    let reply = match response {
        Ok(pet) => pet,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, new_pet: NewPet) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in pet_handler read");

    let response = Pet::update(&conn, new_pet, id).await;

    let reply = match response {
        Ok(_) => 204,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in pet_handler read");

    let response = Pet::delete(&conn, id).await;

    let reply = match response {
        Ok(_) => 204,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in pet_handler read");

    let response = Pet::list(&conn).await;

    let reply = match response {
        Ok(pets) => pets,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}
