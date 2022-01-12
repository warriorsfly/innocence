use crate::{ routes::routes};
use innocence_utils::CONFIG;
use innocence_database::add_database;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

pub(crate) async fn serv() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // 添加跨域
            .wrap(Cors::permissive())
            // 添加日志
            .wrap(Logger::default())
            // .wrap(casbin_middleware.clone())
            // .wrap(InocAuth)
            // 连接数据库
            .configure(add_database)
            // 注册路由
            .configure(routes)
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
