use chrono::{DateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::{books, episodes};

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
    /// 喜爱数量
    pub favorites_count: i32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable)]
#[table_name = "books"]
struct NewBook<'a> {
    pub authors: &'a Vec<String>,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub cover: &'a str,
    pub tags: &'a Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorJson {
    pub id: i32,
    pub name: String,
    pub citizenship: String,
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
