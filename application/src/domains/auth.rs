mod api {
    mod handlers;
    pub mod routes;
}

mod domain {
    mod entity;
    mod reposiory;
    mod service;
}

mod models {
    mod login_models;
}

mod infra {
    mod repository_impl;
}
