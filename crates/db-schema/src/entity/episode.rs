use crate::schema::episodes;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct Episode {
    /// 章节ID
    pub id: i32,
    /// 所属书籍
    pub book: i32,
    /// 姓名
    pub name: String,
    /// 价格
    pub price: i32,
    /// 漫画链接
    pub comics: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct EpisodeHistory {
    /// 章节ID
    pub user_id: i32,
    /// 所属书籍
    pub book_id: i32,
    /// 姓名
    pub episode_id: i32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EpisodeJson {
    /// 章节ID
    pub id: i32,
    /// 所属书籍
    pub book: i32,
    /// 姓名
    pub name: String,
    /// 价格
    pub price: i32,
    /// 是否锁定
    pub readable: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
