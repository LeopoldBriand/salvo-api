use once_cell::sync::OnceCell;
use sqlx::{MySqlPool};

static DATABASE: OnceCell<MySqlPool> = OnceCell::new();

pub fn get_db() -> &'static MySqlPool {
    DATABASE.get().unwrap()
}

pub async fn init_db() {
    let db_uri  = std::env::var("DB_URI").expect("DB_URI must be set in .env file.");
    let pool = MySqlPool::connect(&db_uri).await.unwrap();
    DATABASE.set(pool).unwrap();
}