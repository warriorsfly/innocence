use crate::{
    config::CONFIG,
    constants::{MODEL_PATH, POLICY_PATH},
    database::add_database,
    routes::routes,
};

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    App, HttpServer,
};

pub(crate) async fn serv() -> std::io::Result<()> {
    env_logger::init();

    // let e = Enforcer::new(MODEL_PATH, POLICY_PATH)
    //     .await
    //     .expect("can't read model and policy files");

    HttpServer::new(move || {
        App::new()
            // 添加跨域
            .wrap(Cors::permissive())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .configure(add_database)
            // 注册路由
            .configure(routes)
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
