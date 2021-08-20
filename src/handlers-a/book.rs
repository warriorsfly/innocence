// use actix_web::{
//     web::{block, Data, Form, Json, Path},
//     Error, HttpResponse,
// };

// use crate::{DataContext::DatabaseConnectionPool, entity::Book};
// use crate::schema::books::{self,};

// // / 搜索
// pub async fn search(
//     pool: Data<DatabaseConnectionPool>,
//     param: &str,
// ) -> Result<Vec<Book>, Error> {
//     let conn = &pool.get()?;
//     // let books =
// }
// // / 广告
// // / 每日更新
// // / 排行榜
// // / 免费
// // / 连载中
// // / 已完结

// // / 书架
// // / 阅读记录
// // / 下载列表
