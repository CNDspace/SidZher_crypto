use self::models::*;
use self::schema::data;
use self::schema::data::dsl::*;
use diesel::prelude::*;
use init_lib::{init_db_connection, init_redis_db_connection};
use redis::Commands;

pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

pub fn check_user_sqlite(login_kek: &String, password_kek: &String) -> bool {
    let results = data
        .load::<Creds>(&init_db_connection())
        .expect("Failed to load table!");
    for i in results {
        if i.login.eq(login_kek) && i.password.eq(password_kek) {
            return true;
        }
    }
    false
}

pub fn register_usersqlite(login_kek: String, password_kek: String) -> usize {
    let new_user = UpdateCreds {
        login: login_kek.as_str(),
        password: password_kek.as_str(),
    };

    diesel::insert_into(data::table)
        .values(&new_user)
        .execute(&init_db_connection())
        .expect("Error add new user!")
}

pub fn check_user_redis(username: &String) -> String {
    let mut connection = init_redis_db_connection().unwrap();
    let check_user_db: Option<String> = connection
        .get(username)
        .unwrap_or(Option::from("ERROR".to_string()));
    return if let Some(password_redis) = check_user_db {
        password_redis
    } else {
        "ERROR".to_string()
    };
}

pub fn register_user_redis(
    username_redis: String,
    password_redis: String,
) -> redis::RedisResult<()> {
    let mut connection = init_redis_db_connection().unwrap();
    let _: () = connection.set(username_redis, password_redis)?;
    Ok(())
}
