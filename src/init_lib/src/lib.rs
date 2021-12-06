use sqlx::mysql::MySqlPool;

pub async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    // let pool = MySqlPoolOptions::new()
    //     .max_connections(5)
    //     .connect("jdbc:mysql://root@localhost:3306/users")
    //     .await?;
    let pool = MySqlPool::connect("mysql://root@localhost:3306/users").await?;

    let name = "kekus".to_string();
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let _row = sqlx::query(r#"INSERT INTO creds (name) VALUES (?)"#)
        .bind(name)
        .execute(&pool)
        .await?;

    // assert_eq!(row.0, "kekus".to_string());

    Ok(())
}
