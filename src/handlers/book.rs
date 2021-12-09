use actix_web::web::{block, Data, Json, Path};

use crate::{
    database::{self, Database},
    errors::Error,
    helpers::respond_json,
    utils::Claims,
};

// pub async fn create_book(pool: Data<Database>, entity: Json) -> Result<Book, Error> {}

// / 搜索
pub async fn search(pool: Data<Database>, tag: Path<String>) -> Result<Json<Vec<Book>>, Error> {
    let res = block(move || database::search(&pool, &tag)).await??;
    respond_json(res)
}

pub async fn books_of_weekday(
    pool: Data<Database>,
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

    let res = block(move || database::books_of_weekday(&pool, day)).await??;

    respond_json(res)
}

pub async fn get_book_episodes(
    claims: Option<Claims>,
    pool: Data<Database>,
    entity: Path<String>,
) -> Result<Json<Vec<Episode>>, Error> {
    let book_id = entity
        .parse::<i32>()
        .map_err(|e| Error::BadRequest(e.to_string()))?;
    let mut user = 0;
    if let Some(claims) = claims {
        user = claims.id;
    }

    let res = block(move || database::get_book_episodes(&pool, user, book_id)).await??;
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
