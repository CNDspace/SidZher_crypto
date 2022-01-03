use init_lib::init_connection;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
use self::models::Creds;
use self::models::UpdateCreds;
use crate::schema::data::dsl::data;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn check_user(login_kek: &String, password_kek: &String) -> bool {
    use self::schema::data::dsl::*;

    let results = data
        .load::<Creds>(&init_connection())
        .expect("Failed to load table!");
    for i in results {
        if i.login.eq(login_kek) && i.password.eq(password_kek) {
            return true;
        }
    }
    false
}

pub fn register_user(login_kek: String, password_kek: String) -> usize {
    use self::schema::data;

    let new_user = UpdateCreds {
        login: login_kek.as_str(),
        password: password_kek.as_str(),
    };

    diesel::insert_into(data::table)
        .values(&new_user)
        .execute(&init_connection())
        .expect("Error add new user!")
}
