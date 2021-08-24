use server::serv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod awc;
mod config;
mod constants;
mod model;
mod errors;
mod handlers;
mod helpers;
mod middleware;
mod routes;
mod schema;
mod database;
mod jwt;
mod server;
mod tests;
mod utils;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
