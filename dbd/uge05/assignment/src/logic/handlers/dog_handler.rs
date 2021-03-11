use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use crate::entities::dog::{NewDog, Dog};
use warp;

pub async fn create(new_dog: NewDog) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in dog_handler create");

    let response = Dog::create(&conn, new_dog).await;

    let reply = match response {
        Ok(dog) => {
            println!("{:#?}", &dog);
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
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::read(&conn, id).await;

    let reply = match response {
        Ok(dog) => dog,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, new_dog: NewDog) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::update(&conn, new_dog, id).await;

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
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::delete(&conn, id).await;

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
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::list(&conn).await;

    let reply = match response {
        Ok(dogs) => dogs,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


pub async fn update_procedure(id: i32, new_dog: NewDog) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::update_procedure(&conn, new_dog, id).await;

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


pub async fn read_view(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::read_view(&conn, id).await;

    let reply = match response {
        Ok(dog) => dog,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


pub async fn list_view() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in dog_handler read");

    let response = Dog::list_view(&conn).await;

    let reply = match response {
        Ok(dogs) => dogs,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


