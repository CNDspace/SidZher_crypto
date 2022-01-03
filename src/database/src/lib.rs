use self::models::*;
use self::schema::data;
use self::schema::data::dsl::*;
use diesel::prelude::*;
use init_lib::init_connection;

pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

pub fn check_user(login_kek: &String, password_kek: &String) -> bool {
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
    let new_user = UpdateCreds {
        login: login_kek.as_str(),
        password: password_kek.as_str(),
    };

    diesel::insert_into(data::table)
        .values(&new_user)
        .execute(&init_connection())
        .expect("Error add new user!")
}
