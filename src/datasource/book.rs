use diesel::prelude::*;

use crate::{
    constants::DATE_FORMAT,
    errors::InoError,
    model::{Book, Episode},
};

use super::Database;

pub async fn episodes(database: &Database, id: i32) -> Result<Vec<Episode>, InoError> {
    use crate::schema::episodes::dsl::*;
    let ref mut conn = database.get()?;
    let eps = episodes.filter(book_id.eq(id)).get_results(conn)?;
    Ok(eps)
}

// / 搜索
// / 广告
// / 每日更新
// / 排行榜
// / 免费
// / 连载中
// / 已完结

// / 书架
// / 阅读记录
// / 下载列表
