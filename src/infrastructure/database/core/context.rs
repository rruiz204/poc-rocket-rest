use std::env;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

type PgConn = ConnectionManager<PgConnection>;
type PgPool = Pool<PgConn>;

pub fn context() -> PgPool {
  dotenv().ok();
  let db_uri: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let manager: PgConn = ConnectionManager::<PgConnection>::new(db_uri);

  Pool::builder().build(manager).expect("Failed to create database connection pool")
}