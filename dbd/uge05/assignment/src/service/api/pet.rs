#[macro_export]
macro_rules! create_pet {
    () => {
        pet_routes::create().and_then(pet_handler::create)
    };
}

#[macro_export]
macro_rules! read_pet {
    () => {
        pet_routes::read().and_then(pet_handler::read)
    };
}

#[macro_export]
macro_rules! update_pet {
    () => {
        pet_routes::update().and_then(pet_handler::update)
    };
}

#[macro_export]
macro_rules! delete_pet {
    () => {
        pet_routes::delete().and_then(pet_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_pet {
    () => {
        pet_routes::list().and_then(pet_handler::list)
    };
}
