use actix_web::web::{block, Data, Json, Path};

use innocence_db_schema::{
    entity::{Book, EpisodeJson},
    repository, Database,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::helpers::respond_json;
use innocence_utils::{Claims, Error};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewBookForm {
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
pub async fn search(pool: Data<Database>, tag: Path<String>, page_index: Path<i64>,
    page_size: Path<i64>,) -> Result<Json<(Vec<Book>,i64)>, Error> {
    let res = block(move || repository::search(&pool, &tag,
            page_index.into_inner(),
            page_size.into_inner(),)).await??;
    respond_json(res)
}

pub async fn books_of_weekday(
    pool: Data<Database>,
    weekday: Path<String>,
    page_index: Path<i64>,
    page_size: Path<i64>,
) -> Result<Json<(Vec<Book>,i64)>, Error> {
    let res = block(move || repository::books_of_weekday(&pool, &weekday,page_index.into_inner(),
            page_size.into_inner(),)).await??;

    respond_json(res)
}

pub async fn get_book_episodes(
    claims: Option<Claims>,
    pool: Data<Database>,
    slug: Path<String>,
    page_index: Path<i64>,
    page_size: Path<i64>,
) -> Result<Json<(Vec<EpisodeJson>, i64)>, Error> {
    let user = claims.map(|cl| cl.sub).unwrap_or(0);

    let res = block(move || {
        repository::get_book_episodes(
            &pool,
            user,
            slug.as_str(),
            page_index.into_inner(),
            page_size.into_inner(),
        )
    })
    .await??;
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
