use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use crate::entities::cat::{NewCat, Cat};
use warp;

pub async fn create(new_cat: NewCat) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in cat_handler create");

    let response = Cat::create(&conn, new_cat).await;

    let reply = match response {
        Ok(cat) => {
            println!("{:#?}", &cat);
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
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::read(&conn, id).await;

    let reply = match response {
        Ok(cat) => cat,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, new_cat: NewCat) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::update(&conn, new_cat, id).await;

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
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::delete(&conn, id).await;

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
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::list(&conn).await;

    let reply = match response {
        Ok(cats) => cats,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


pub async fn update_procedure(id: i32, new_cat: NewCat) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = get_pool()
        .await
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::update_procedure(&conn, new_cat, id).await;

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
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::read_view(&conn, id).await;

    let reply = match response {
        Ok(cat) => cat,
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
        .expect("Pool could not be created in cat_handler read");

    let response = Cat::list_view(&conn).await;

    let reply = match response {
        Ok(cats) => cats,
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


