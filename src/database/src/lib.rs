/*
    Библиотека используется для обращения к базе данных
*/

use init_lib::init_redis_db_connection;
use redis::Commands;
// use init_lib::{init_db_connection, init_redis_db_connection};
// use self::models::*;
// use self::schema::data;
// use self::schema::data::dsl::*;
// use diesel::prelude::*;

// pub mod models;
// pub mod schema;

// #[macro_use]
// extern crate diesel;

// pub fn check_user_sqlite(login_kek: &String, password_kek: &String) -> bool {
//     let results = data
//         .load::<Creds>(&init_db_connection())
//         .expect("Failed to load table!");
//     for i in results {
//         if i.login.eq(login_kek) && i.password.eq(password_kek) {
//             return true;
//         }
//     }
//     false
// }

// pub fn register_usersqlite(login_kek: String, password_kek: String) -> usize {
//     let new_user = UpdateCreds {
//         login: login_kek.as_str(),
//         password: password_kek.as_str(),
//     };
//
//     diesel::insert_into(data::table)
//         .values(&new_user)
//         .execute(&init_db_connection())
//         .expect("Error add new user!")
// }

// функция используется для проверки пользователя в базе данных redis
pub fn check_user_redis(username: &String) -> String {
    // создаём соединение с базой данных
    let mut connection = init_redis_db_connection().unwrap();
    // используем соединение с базой данных для запроса и проверки пользователя,
    // если не смогли получить данные о пользователе, то возвращаем ERROR
    let check_user_db: Option<String> = connection
        .get(username)
        .unwrap_or(Option::from("ERROR".to_string()));
    // если смогли получить пароль из базы данных, то возвращаем пароль
    return if let Some(password_redis) = check_user_db {
        password_redis
    // иначе возвращаем ERROR
    } else {
        "ERROR".to_string()
    };
}

// функция используется для регистрации нового пользователя
pub fn register_user_redis(username_redis: String, password_redis: String) -> bool {
    // создаём соединение с базой данных
    let mut connection = init_redis_db_connection().unwrap();
    // создаём запрос к базе данных, где создаём нового пользователя
    let _: () = connection
        .set(username_redis, password_redis)
        .expect("Failed to add user!");
    return true;
}
