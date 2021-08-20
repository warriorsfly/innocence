use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::{books, episodes};

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct Book {
    /// ID
    pub id: i32,
    pub slug: String,
    /// 名称
    pub name: String,
    /// 封面图片
    pub cover: String,
    /// 描述
    pub description: String,
    /// 作者
    // pub authors: Vec<AuthorJson>,
    /// 标签
    pub tags: Vec<String>,
    /// 章节
    // pub episodes: Vec<EpisodeJson>,
    /// 喜爱数量
    pub favorite_count: i32,

    pub score: f32,
    /// 是否连载
    pub serialing: bool,
}
#[derive(Debug, Insertable)]
#[table_name = "books"]
struct NewBook<'a> {
    pub author_id: &'a i32,
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
