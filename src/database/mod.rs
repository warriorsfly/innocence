use actix_web::web::{self, Data};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use crate::{jwt::Claims, config::CONFIG};

mod book;
mod pagination;
mod user;

pub use self::{book::*,  pagination::*, user::*};

pub type Database = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;


pub fn add_database(cfg: &mut web::ServiceConfig) {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    let database = Pool::builder().build(manager).expect("database_url error");
    cfg.app_data(Data::new(database));
}

