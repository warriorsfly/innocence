use std::ops::Add;

use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    constants::DATE_FORMAT,
    errors::Error,
    schema::{books, episodes},
};

use super::Connection;

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
pub struct Book {
    /// ID
    pub id: i32,
    pub authors: Vec<String>,
    pub slug: String,
    /// 名称
    pub name: String,
    /// 封面图片
    pub cover: String,
    /// 描述
    pub description: String,
    /// 标签
    pub tags: Vec<String>,
    pub day_of_week: i32,
    /// 喜爱数量
    pub favorites_count: i32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub authors: &'a Vec<String>,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub cover: &'a str,
    pub tags: &'a Vec<String>,
    pub day_of_week: &'a i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bill {
    pub id: i32,
    pub user: i32,
}

#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct Episode {
    /// 章节ID
    pub id: i32,
    /// 所属书籍
    pub book: i32,
    /// 姓名
    pub name: String,
    /// 漫画链接
    pub comics: Vec<String>,
}

// pub fn create_book<'a>(conn: &'a mut Connection, entity: &'a NewBook) -> Result<Book, Error> {
//     use crate::schema::books::{self, dsl::*};

//     diesel::insert_into(books)
//         .values(entity)
//         .get_result(conn)
//         .map_err(|err| Error::DataBaseError(err.to_string()))
// }

pub fn favorites(conn: &Connection, id: i32) -> Result<Vec<Book>, Error> {
    use crate::schema::{
        books::{self, dsl::*},
        favorite_books::{self, dsl::*},
    };
    let list = favorite_books
        .filter(favorite_books::user_id.eq(id))
        .inner_join(books.on(favorite_books::book_id.eq(books::id)))
        .select(books::all_columns)
        .get_results(conn)?;
    Ok(list)
}

pub fn episodes(conn: &mut Connection, id: i32) -> Result<Vec<Episode>, Error> {
    use crate::schema::episodes::dsl::*;
    let eps = episodes.filter(book_id.eq(id)).get_results(conn)?;
    Ok(eps)
}

pub fn search(conn: &mut Connection, tag: &str) -> Result<Vec<Book>, Error> {
    use crate::schema::books::{self, dsl::*};

    let res = books
        .filter(books::name.contains(tag))
        .or(books::tags.contains(tag))
        .get_results(conn)?;
    Ok(res)
}

pub async fn day_of_week(conn: &mut Connection, dow: i32) {
    use crate::schema::books::{self, dsl::*};

    let res: Vec<Book> = books
        .filter(books::day_of_week.eq(dow))
        .order_by(updated_at.desc())
        .get_results(conn)?;
    Ok(res)
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
