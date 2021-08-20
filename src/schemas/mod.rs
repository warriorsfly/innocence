use actix_web::web::{self, Data};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use juniper::Context;

use crate::config::CONFIG;

mod book;
mod pagination;
mod root;
mod user;

pub use self::{book::*, pagination::*, root::*, user::*};

pub type Database = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DataContext {
    pub(crate) database: Database,
}

impl Context for DataContext {}

pub fn add_database(cfg: &mut web::ServiceConfig) {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    let database = Pool::builder().build(manager).expect("database_url error");
    cfg.app_data(Data::new(database));
}
pub fn add_graphql(cfg: &mut web::ServiceConfig) {
    let schema = init_schema();
    cfg.app_data(Data::new(schema));
}
