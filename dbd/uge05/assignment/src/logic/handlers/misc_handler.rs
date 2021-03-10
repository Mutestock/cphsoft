use crate::misc::redis_interactions::swap_restricted_user;
use warp;


// This program utilizes two separate users. One with restricted access and one with full access.
// This handler is responsible for swapping the user to demonstrate access changes.
pub async fn user_switch() -> Result<impl warp::Reply, warp::Rejection>{

    let response = swap_restricted_user();

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


