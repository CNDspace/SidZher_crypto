use init_lib::init_connection;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::UpdateCreds;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn check_user() {
    //todo
}

pub fn register_user() {
    //todo
}
