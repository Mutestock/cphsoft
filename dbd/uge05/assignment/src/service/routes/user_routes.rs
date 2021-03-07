use warp::{filters::BoxedFilter, path, Filter};
use crate::entities::account::user::User;

fn path_prefix() -> BoxedFilter<()>{
    path!("api" / "user" / ..)
        .boxed()
}

pub fn create() -> BoxedFilter<(User,)>{
    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}