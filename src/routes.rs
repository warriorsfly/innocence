use actix_files::Files;
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
                        "/{slug}/episodes/{page_index}/{page_size}",
                        web::get().to(book::get_book_episodes),
                    )
                    .route("/search/{param}", web::get().to(book::search))
                    .route("/update/{weekday}/{page_index}/{page_size}", web::get().to(book::books_of_weekday)),
            ),
    )
    .service(Files::new("/static", "static").prefer_utf8(true));
}
