use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_connection_pool(database_url: &str, max_size: u32) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .max_size(max_size)
        .build(manager)
        .expect("Failed to create database connection pool")
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, r2d2::Error> {
    pool.get()
}
