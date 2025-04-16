use std::env;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgConn = ConnectionManager<PgConnection>;
pub type PgPooled = PooledConnection<PgConn>;
pub type PgPool = Pool<PgConn>;

pub struct Context {
  pub pool: PgPool,
}

impl Context {
  fn load_uri() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
  }

  fn build_pool(db_uri: String) -> PgPool {
    let manager: PgConn = ConnectionManager::<PgConnection>::new(db_uri);
    Pool::builder().build(manager).expect("Failed to create database connection pool")
  }

  pub fn new() -> Self {
    let db_uri: String = Self::load_uri();
    let pool: PgPool = Self::build_pool(db_uri);
    
    Self { pool }
  }
}