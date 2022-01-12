use server::serv;



mod handlers;
mod helpers;
mod routes;
mod server;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
