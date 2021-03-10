use warp::{filters::BoxedFilter, path, Filter};

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "misc" / ..).boxed()
}

pub fn user_swap() -> BoxedFilter<()> {
    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}
