#[macro_export]
macro_rules! create_user {
    () => {
        user_routes::create()
            .and_then(user_handler::create)
    };
}