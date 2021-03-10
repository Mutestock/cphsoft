#[macro_export]
macro_rules! create_city {
    () => {
        city_routes::create().and_then(city_handler::create)
    };
}

#[macro_export]
macro_rules! read_city {
    () => {
        city_routes::read().and_then(city_handler::read)
    };
}

#[macro_export]
macro_rules! update_city {
    () => {
        city_routes::update().and_then(city_handler::update)
    };
}

#[macro_export]
macro_rules! delete_city {
    () => {
        city_routes::delete().and_then(city_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_city {
    () => {
        city_routes::list().and_then(city_handler::list)
    };
}
