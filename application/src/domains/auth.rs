mod api {
    mod handlers;
    pub mod routes;
}

mod domain {
    mod entity;
    mod reposiory;
    mod service;
}

mod model {
    mod userinfo;
}

mod infra {
    mod repository_impl;
}
