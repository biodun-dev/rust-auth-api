use sea_orm_migration::prelude::*;
use migration::Migrator;

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sea_orm::Database::connect(&url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
}
