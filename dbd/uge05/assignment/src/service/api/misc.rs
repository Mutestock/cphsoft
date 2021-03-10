
#[macro_export]
macro_rules! user_swap {
    () => {
        misc_routes::user_swap().and_then(misc_handler::user_switch)
    };
}


