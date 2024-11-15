use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};


pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)] // allows to clone this object, cause we can't pass it like that by default
pub struct AppState {
    pub pools: DbPool,
}
