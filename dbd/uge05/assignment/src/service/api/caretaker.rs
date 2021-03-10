#[macro_export]
macro_rules! create_caretaker {
    () => {
        caretaker_routes::create().and_then(caretaker_handler::create)
    };
}

#[macro_export]
macro_rules! read_caretaker {
    () => {
        caretaker_routes::read().and_then(caretaker_handler::read)
    };
}

#[macro_export]
macro_rules! update_caretaker {
    () => {
        caretaker_routes::update().and_then(caretaker_handler::update)
    };
}

#[macro_export]
macro_rules! delete_caretaker {
    () => {
        caretaker_routes::delete().and_then(caretaker_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_caretaker {
    () => {
        caretaker_routes::list().and_then(caretaker_handler::list)
    };
}
