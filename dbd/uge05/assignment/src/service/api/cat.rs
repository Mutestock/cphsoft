#[macro_export]
macro_rules! create_cat {
    () => {
        cat_routes::create().and_then(cat_handler::create)
    };
}

#[macro_export]
macro_rules! read_cat {
    () => {
        cat_routes::read().and_then(cat_handler::read)
    };
}

#[macro_export]
macro_rules! update_cat {
    () => {
        cat_routes::update().and_then(cat_handler::update)
    };
}

#[macro_export]
macro_rules! delete_cat {
    () => {
        cat_routes::delete().and_then(cat_handler::delete)
    };
}

#[macro_export]
macro_rules! read_list_cat {
    () => {
        cat_routes::list().and_then(cat_handler::list)
    };
}

#[macro_export]
macro_rules! read_view_cat {
    () => {
        cat_routes::read_view().and_then(cat_handler::read_view)
    };
}

#[macro_export]
macro_rules! update_procedure_cat {
    () => {
        cat_routes::update_procedure().and_then(cat_handler::update_procedure)
    };
}

#[macro_export]
macro_rules! read_list_view_cat {
    () => {
        cat_routes::list_view().and_then(cat_handler::list_view)
    };
}


