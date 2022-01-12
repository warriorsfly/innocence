use server::serv;



mod errors;
mod handlers;
mod helpers;
mod routes;
mod server;
mod tests;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
