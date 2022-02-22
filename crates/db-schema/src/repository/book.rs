use diesel::prelude::*;
use innocence_utils::{Error,Paginate};

use crate::{
    entity::{Book, Episode, NewBook, EpisodeJson},
    Database,
};

pub fn create_book<'a>(pool: &'a Database, entity: &'a NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::*;
    let ref mut conn = pool.get()?;
    diesel::insert_into(books)
        .values(entity)
        .get_result(conn)
        .map_err(|err| Error::DataBaseError(err.to_string()))
}

pub fn get_favorite_books(pool: &Database, entity_id: i32) -> Result<Vec<Book>, Error> {
    use crate::schema::{
        books::{self, dsl::*},
        favorite_books::{self, dsl::*},
    };

    let ref mut conn = pool.get()?;
    let list = favorite_books
        .filter(favorite_books::user_id.eq(entity_id))
        .inner_join(books.on(favorite_books::book_id.eq(books::id)))
        .select(books::all_columns)
        .get_results(conn)?;
    Ok(list)
}

pub fn get_book_episodes(pool: &Database, user: i32, slug: &str,page_index:i64,page_size:i64) -> Result<(Vec<EpisodeJson>,i64), Error> {
    use crate::schema::{books,episode_historys, episodes};
    let ref mut conn = pool.get()?;
    let (eps,total_pages)= episodes::table
    .left_join(episode_historys::table.on(episode_historys::episode_id.eq(episodes::id).and(episode_historys::user_id.eq(user))))
    .inner_join(books::table.on(books::id.eq(episodes::book_id).and(books::slug.eq(slug))))
    .select((episodes::all_columns, episode_historys::episode_id.nullable()))
    .paginate(page_index).per_page(page_size)
        .load_and_count_pages::<(Episode,Option<i32>)>(conn)?;
    
    // Ok(eps)
     let res = eps.iter().map
     (|(e, h)| EpisodeJson {
            id: e.id,
            book: e.book,
            name: e.name.clone(),
            price: e.price,
            readable: h.is_some(),
            created_at: e.created_at,
            updated_at: e.updated_at,
        });
    Ok((res.collect(),total_pages))
}

pub fn search(pool: &Database, tag: &str) -> Result<Vec<Book>, Error> {
    use crate::schema::books::dsl::*;
    let ref mut conn = pool.get()?;
    let res = books
        .filter(name.like(&tag))
        // .f(tags.contains(tag))
        .get_results(conn)?;
    Ok(res)
}

pub fn books_of_weekday(pool: &Database, wd: &str) -> Result<Vec<Book>, Error> {
    use crate::schema::books::dsl::*;
    let ref mut conn = pool.get()?;
    let res: Vec<Book> = books
        .filter(weekday.eq(wd))
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
