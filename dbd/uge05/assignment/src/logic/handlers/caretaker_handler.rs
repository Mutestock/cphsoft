use crate::connection::conn::get_pool;
use crate::entities::caretaker::{Caretaker, NewCaretaker};
use crate::entities::entity_utils::CRUD;
use warp;

pub async fn create(new_caretaker: NewCaretaker) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in caretaker_handler create");

    let response = Caretaker::create(&conn, new_caretaker).await;

    let reply = match response {
        Ok(caretaker) => {
            println!("{:#?}", &caretaker);
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
        .expect("Pool could not be created in caretaker_handler read");

    let response = Caretaker::read(&conn, id).await;

    let reply = match response {
        Ok(caretaker) => caretaker,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update(
    id: i32,
    new_caretaker: NewCaretaker,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in caretaker_handler read");

    let response = Caretaker::update(&conn, new_caretaker, id).await;

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
        .expect("Pool could not be created in caretaker_handler read");

    let response = Caretaker::delete(&conn, id).await;

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
        .expect("Pool could not be created in caretaker_handler read");

    let response = Caretaker::list(&conn).await;

    let reply = match response {
        Ok(caretakers) => caretakers,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}
