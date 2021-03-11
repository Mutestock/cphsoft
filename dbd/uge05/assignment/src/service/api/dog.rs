#[macro_export]
macro_rules! create_dog {
    () => {
        dog_routes::create().and_then(dog_handler::create)
    };
}

#[macro_export]
macro_rules! read_dog {
    () => {
        dog_routes::read().and_then(dog_handler::read)
    };
}

#[macro_export]
macro_rules! update_dog {
    () => {
        dog_routes::update().and_then(dog_handler::update)
    };
}

#[macro_export]
macro_rules! delete_dog {
    () => {
        dog_routes::delete().and_then(dog_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_dog {
    () => {
        dog_routes::list().and_then(dog_handler::list)
    };
}

#[macro_export]
macro_rules! read_view_dog {
    () => {
        dog_routes::read_view().and_then(dog_handler::read_view)
    };
}

#[macro_export]
macro_rules! update_procedure_dog {
    () => {
        dog_routes::update_procedure().and_then(dog_handler::update_procedure)
    };
}

#[macro_export]
macro_rules! read_list_view_dog {
    () => {
        dog_routes::list_view().and_then(dog_handler::list_view)
    };
}

