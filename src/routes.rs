use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::resource("/graphql")
    //         .route(web::post().to(graphql))
    //         .route(web::get().to(graphql)),
    // )
    // .service(web::resource("/playground").route(web::get().to(playground_handler)))
    // .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)));
}
