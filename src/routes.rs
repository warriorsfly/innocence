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
            .service(
                web::scope("/books")
                    .route(
                        "/{book_id}/episodes",
                        web::get().to(book::get_book_episodes),
                    )
                    .route("/search/{param}", web::get().to(book::search))
                    .route("/{weekday}/list", web::get().to(book::day_of_week)),
                // .route("/weekday/{weekday}", web::get().to(book::day_of_week)),
            ),
    );
}
