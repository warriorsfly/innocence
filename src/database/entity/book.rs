use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::books;

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
    pub weekday: String,
    /// 喜爱数量
    pub favorites_count: i32,
    pub completed: bool,
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
    pub weekday: &'a str,
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Bill {
    pub id: i32,
    pub user: i32,
}

