use crate::{
    config::CONFIG,
    routes::routes,
    schemas::{add_database, add_graphql},
};

use actix_web::{middleware::Logger, App, HttpServer};

pub(crate) async fn serv() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            // 添加跨域
            // .wrap(Cors::permissive())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .configure(add_database)
            .configure(add_graphql)
            // 注册路由
            .configure(routes)
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
