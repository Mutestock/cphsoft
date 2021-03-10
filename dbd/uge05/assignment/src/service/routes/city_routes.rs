use crate::entities::city::NewCity;
use warp::{filters::BoxedFilter, path, Filter};
#[macro_use]
use crate::service::api::city;
use crate::create_city;
use crate::logic::handlers::city_handler;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "city" / ..).boxed()
}

pub fn create() -> BoxedFilter<(NewCity,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn read() -> BoxedFilter<(i32,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn update() -> BoxedFilter<(i32, NewCity)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .and(json_body)
        .boxed()
}

pub fn delete() -> BoxedFilter<(i32,)> {
    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}
