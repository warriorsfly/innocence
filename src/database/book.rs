use diesel::prelude::*;

use crate::{
    constants::DATE_FORMAT,
    errors::Error,
    model::{Book, Episode},
};

use super::Database;

pub fn favorites(database: &Database, id: i32) -> Result<Vec<Book>, Error> {
    use crate::schema::{
        books::{self, dsl::*},
        favorite_books::{self, dsl::*},
    };
    let ref mut conn = database.get()?;
    let list = favorite_books
        .filter(favorite_books::user_id.eq(id))
        .inner_join(books.on(favorite_books::book_id.eq(books::id)))
        .select(books::all_columns)
        .get_results(conn)?;
    Ok(list)
}

pub async fn episodes(database: &Database, id: i32) -> Result<Vec<Episode>, Error> {
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
