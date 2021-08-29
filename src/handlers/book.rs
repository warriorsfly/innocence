use actix_web::HttpResponse;
use actix_web::{
    web::{block, Data, Form, Json, Path},
    Error,
};

use crate::{
    database::{self, Book, Database},
    helpers::respond_json,
};

pub async fn create_book(database: Data<Database>, entity: Json) -> Result<Book, Error> {}

// / 搜索
pub async fn search(database: Data<Database>, param: &str) -> Result<Json<Vec<Book>>, Error> {
    let ref conn = database.get()?;

    let res = database::search(conn, param)?;

    respond_json(res)
    // let books =
}
// // / 广告
// // / 每日更新
// // / 排行榜
// // / 免费
// // / 连载中
// // / 已完结

// // / 书架
// // / 阅读记录
// // / 下载列表
