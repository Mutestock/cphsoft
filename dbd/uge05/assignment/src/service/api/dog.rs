#[macro_export]
macro_rules! create_dog {
    () => {
        dog_routes::create()
            .and_then(dog_handler::create)
    };
}

#[macro_export]
macro_rules! read_dog {
    () => {
        dog_routes::read()
            .and_then(dog_handler::read)
    };
}

#[macro_export]
macro_rules! update_dog {
    () => {
        dog_routes::update()
            .and_then(dog_handler::update)
    };
}

#[macro_export]
macro_rules! delete_dog {
    () => {
        dog_routes::delete()
            .and_then(dog_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_dog {
    () => {
        dog_routes::read_list()
            .and_then(dog_handler::read_list)
    };
}
