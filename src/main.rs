use server::serv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod database;
mod errors;
mod handlers;
mod helpers;
mod middleware;
mod utils;
mod routes;
mod schema;
mod server;
mod tests;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
