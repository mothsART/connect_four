extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;
pub mod grid;

#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
//use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub const ADDR: &'static str = "0.0.0.0";
pub const PORT: &'static str = "3012";

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/*
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
*/
