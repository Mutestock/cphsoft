use crate::entities::dog::NewDog;
use warp::{filters::BoxedFilter, path, Filter};

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "dog" / ..).boxed()
}

pub fn create() -> BoxedFilter<(NewDog,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn read() -> BoxedFilter<(i32,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn update() -> BoxedFilter<(i32, NewDog)> {
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

pub fn read_view() -> BoxedFilter<(i32,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("extended"))
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn update_procedure() -> BoxedFilter<(i32, NewDog)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(path_prefix())
        .and(warp::path("extended"))
        .and(warp::path::param::<i32>())
        .and(json_body)
        .boxed()
}

pub fn list_view() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("extended"))
        .and(warp::path::end())
        .boxed()
}
