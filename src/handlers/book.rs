use actix_web::web::{Data, Json, Path};

use crate::{
    database::{self, Book, Database, Episode},
    errors::Error,
    helpers::respond_json,
    plugins::Claims,
};

// pub async fn create_book(database: Data<Database>, entity: Json) -> Result<Book, Error> {}

// / 搜索
pub async fn search(database: Data<Database>, tag: Path<String>) -> Result<Json<Vec<Book>>, Error> {
    let ref mut conn = database.get()?;

    let res = database::search(conn, &tag).await?;

    respond_json(res)
    // let books =
}

pub async fn books_of_weekday(
    database: Data<Database>,
    weekday: Path<String>,
) -> Result<Json<Vec<Book>>, Error> {
    let day = match weekday.to_lowercase().as_str() {
        "mon" => 1,
        "tue" => 2,
        "wed" => 3,
        "thu" => 4,
        "fri" => 5,
        "sat" => 6,
        "sun" => 7,
        _ => -1,
    };

    if day < 0 {
        return Err(Error::BadRequest("error week day request".to_string()));
    }
    let ref mut conn = database.get()?;

    let res = database::books_of_weekday(conn, day).await?;

    respond_json(res)
}

pub async fn get_book_episodes(
    claims: Option<Claims>,
    database: Data<Database>,
    entity: Path<String>,
) -> Result<Json<Vec<Episode>>, Error> {
    let book_id = entity
        .parse::<i32>()
        .map_err(|e| Error::BadRequest(e.to_string()))?;
    let ref mut conn = database.get()?;
    let res = database::get_book_episodes(conn, book_id)?;
    respond_json(res)
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
