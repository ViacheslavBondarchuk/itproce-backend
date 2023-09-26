use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::environments;

pub fn connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    Pool::builder()
        .max_size(environments::database::connection_pool_size())
        .build(ConnectionManager::<PgConnection>::new(environments::database::url()))
        .expect("Cannot create database connection pool")
}
