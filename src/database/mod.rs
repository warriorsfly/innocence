use crate::config::CONFIG;
use actix_web::web::{Data, ServiceConfig};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

mod dao;
mod entity;
// mod pagination;

pub(crate) use self::{dao::*, entity::*};

pub type Database = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn add_database(cfg: &mut ServiceConfig) {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    let database = Pool::builder().build(manager).expect("database_url error");
    cfg.app_data(Data::new(database));
}
