use crate::entities::account::user::User;
use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use warp;


pub async fn create(new_user: User) -> Result<impl warp::Reply, warp::Rejection>{
    let conn = get_pool()
        .await
        .expect("cake");
        
    let response = User::create(&conn, new_user)
        .await;

    let reply = match response {
        Ok(user) => {
            println!("{:#?}", &user);
            204
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}

pub async fn read(){
    unimplemented!();
}

pub async fn update(){
    unimplemented!();
}

pub async fn delete(){
    unimplemented!();
}

pub async fn list(){
    unimplemented!();
}

