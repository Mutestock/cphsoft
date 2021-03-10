#[macro_export]
macro_rules! create_vet {
    () => {
        vet_routes::create().and_then(vet_handler::create)
    };
}

#[macro_export]
macro_rules! read_vet {
    () => {
        vet_routes::read().and_then(vet_handler::read)
    };
}

#[macro_export]
macro_rules! update_vet {
    () => {
        vet_routes::update().and_then(vet_handler::update)
    };
}

#[macro_export]
macro_rules! delete_vet {
    () => {
        vet_routes::delete().and_then(vet_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_vet {
    () => {
        vet_routes::list().and_then(vet_handler::list)
    };
}
