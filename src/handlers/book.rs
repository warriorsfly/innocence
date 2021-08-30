use actix_web::web::{Data, Json, Path};

use crate::{
    database::{self, Book, Database},
    errors::Error,
    helpers::respond_json,
};

// pub async fn create_book(database: Data<Database>, entity: Json) -> Result<Book, Error> {}

// / 搜索
pub async fn search(database: Data<Database>, tag: Path<String>) -> Result<Json<Vec<Book>>, Error> {
    let ref mut conn = database.get()?;

    let res = database::search(conn, &tag).await?;

    respond_json(res)
    // let books =
}

pub async fn day_of_week(
    database: Data<Database>,
    weekday: Path<String>,
) -> Result<Json<Vec<Book>>, Error> {
    let day = match weekday.as_str() {
        "mon" => 1,
        "tue" => 2,
        "wed" => 3,
        "thu" => 4,
        "fri" => 5,
        "sat" => 6,
        "sun" => 7,
        _ => -1,
    };

    if day == -1 {
        return Err(Error::BadRequest("error week day request".to_string()));
    }

    let ref mut conn = database.get()?;

    let res = database::day_of_week(conn, day).await?;

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
