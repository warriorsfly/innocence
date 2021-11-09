use diesel::prelude::*;

use crate::errors::Error;

use super::{Book, Connection, Episode, EpisodeHistory, NewBook};

pub fn create_book<'a>(conn: &'a mut Connection, entity: &'a NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::*;

    diesel::insert_into(books)
        .values(entity)
        .get_result(conn)
        .map_err(|err| Error::DataBaseError(err.to_string()))
}

pub fn get_favorite_books(conn: &mut Connection, entity_id: i32) -> Result<Vec<Book>, Error> {
    use crate::schema::{
        books::{self, dsl::*},
        favorite_books::{self, dsl::*},
    };
    let list = favorite_books
        .filter(favorite_books::user_id.eq(entity_id))
        .inner_join(books.on(favorite_books::book_id.eq(books::id)))
        .select(books::all_columns)
        .get_results(conn)?;
    Ok(list)
}

pub fn get_book_episodes(
    conn: &mut Connection,
    user: i32,
    book: i32,
) -> Result<Vec<Episode>, Error> {
    use crate::schema::{episode_historys, episodes};

    let eps = episodes::table
        .filter(episodes::book_id.eq(book))
        .get_results(conn)?;
    let mine_eps: Vec<EpisodeHistory> = episode_historys::table
        .filter(episode_historys::user_id.eq(user))
        .filter(episode_historys::book_id.eq(book))
        .get_results(conn)?;
    Ok(eps)
}

pub async fn search(conn: &mut Connection, tag: &str) -> Result<Vec<Book>, Error> {
    use crate::schema::books::dsl::*;

    let res = books
        .filter(name.like(&tag))
        // .f(tags.contains(tag))
        .get_results(conn)?;
    Ok(res)
}

pub async fn books_of_weekday(conn: &mut Connection, dow: i32) -> Result<Vec<Book>, Error> {
    use crate::schema::books::dsl::*;

    let res: Vec<Book> = books
        .filter(day_of_week.eq(dow))
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
