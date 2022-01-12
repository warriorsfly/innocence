use actix_web::web::{block, Data, Json, Path};

use innocence_database::{
    dao,
    entity::{Book, Episode},
    Database,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::helpers::respond_json;
use innocence_utils::{Claims, Error};


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewBookInput {
    pub authors: String,
    pub slug: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub cover: String,
    pub tags: Vec<String>,
    pub day_of_week: i32,
}

// pub async fn create_book(pool: Data<Database>, entity: Json) -> Result<Book, Error> {}

// / 搜索
pub async fn search(pool: Data<Database>, tag: Path<String>) -> Result<Json<Vec<Book>>, Error> {
    let res = block(move || dao::search(&pool, &tag)).await??;
    respond_json(res)
}

pub async fn books_of_weekday(
    pool: Data<Database>,
    weekday: Path<String>,
) -> Result<Json<Vec<Book>>, Error> {
    let res = block(move || dao::books_of_weekday(&pool, &weekday)).await??;

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

    let res = block(move || dao::get_book_episodes(&pool, user, book_id)).await??;
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
