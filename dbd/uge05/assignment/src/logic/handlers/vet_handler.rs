use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use crate::entities::vet::{NewVet, Vet};
use warp;

pub async fn create(new_vet: NewVet) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in vet_handler create");

    let response = Vet::create(&conn, new_vet).await;

    let reply = match response {
        Ok(vet) => {
            println!("{:#?}", &vet);
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
        .expect("Pool could not be created in vet_handler read");

    let response = Vet::read(&conn, id).await;

    let reply = match response {
        Ok(vet) => vet,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, new_vet: NewVet) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in vet_handler read");

    let response = Vet::update(&conn, new_vet, id).await;

    let reply = match response {
        Ok(vet) => 204,
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
        .expect("Pool could not be created in vet_handler read");

    let response = Vet::delete(&conn, id).await;

    let reply = match response {
        Ok(vet) => 204,
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
        .expect("Pool could not be created in vet_handler read");

    let response = Vet::list(&conn).await;

    let reply = match response {
        Ok(vets) => vets,
        Err(e) => {
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}
