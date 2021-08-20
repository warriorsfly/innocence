use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};

use crate::entity::{Book, Episode};

use super::DataContext;

#[graphql_object(context = DataContext)]
impl Book {
    pub fn id(&self) -> &i32 {
        &self.id
    }
    #[graphql(description = "book's slug url")]
    pub fn slug(&self) -> &str {
        &self.slug
    }

    #[graphql(description = "book's name")]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[graphql(description = "user's email")]
    pub fn cover(&self) -> &str {
        &self.cover
    }
    #[graphql(description = "user's bio")]
    pub fn description(&self) -> &str {
        &self.description
    }
    #[graphql(description = "user's avatar")]
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    #[graphql(description = "all the episodes in the book")]
    pub async fn episodes(&self, ctx: &DataContext) -> FieldResult<Vec<Episode>> {
        use crate::schema::episodes::dsl::*;
        let ref mut conn = ctx.database.get()?;
        let eps = episodes.filter(book_id.eq(&self.id)).get_results(conn)?;
        Ok(eps)
    }
}

#[graphql_object(context = DataContext)]
impl Episode {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    #[graphql(description = "episode name")]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[graphql(description = "comic urls")]
    pub fn comics(&self) -> &Vec<String> {
        &self.comics
    }
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
