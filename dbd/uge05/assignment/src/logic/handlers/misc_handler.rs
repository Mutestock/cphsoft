use crate::misc::sql_interactions::user_swap;
use crate::connection::conn::get_pool;
use warp;


// This program utilizes two separate users. One with restricted access and one with full access.
// This handler is responsible for swapping the user to demonstrate access changes.
pub async fn postgres_user_switch() -> Result<impl warp::Reply, warp::Rejection>{
    let conn = get_pool()
        .await
        .expect("Pool could not be created in postgres_user_switch create");

    let response = user_swap(&conn).await;

    let reply = match response {
        Ok(_) => {
            "User has been swapped. Check restricted route access"
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}


