use actix_web::web;

use crate::handlers::{book, user};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/user")
                    .route("signup", web::post().to(user::signup))
                    .route("login", web::post().to(user::login)),
            )
            // .service(
            //     web::scope("/users").route("/{id}", web::get().to(user::get_user)), // .route("", web::get().to(get_users))
            // )
            .service(
                web::scope("/books")
                    .route("/search/{param}", web::post().to(book::search))
                    .route("/weekday/{weekday}", web::get().to(book::day_of_week)),
                // .route("/weekday/{weekday}", web::get().to(book::day_of_week)),
            ),
    );
}
