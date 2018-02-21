extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate uuid;

pub mod schema;
pub mod models;
pub mod grid;

use diesel::{insert_into, update, delete};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use grid::{Grid};
use models::{User, NewUser, GameInProgress, NewGameInProgress};
use schema::users::*;
use schema::users::dsl::users;
use schema::game_in_progress::*;
use schema::game_in_progress::dsl::game_in_progress;

pub const ADDR: &'static str = "0.0.0.0";
pub const PORT: &'static str = "3012";

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct ConnectFourDataBaseStruct {
    pub connection: SqliteConnection,
    pub game_id : u32
}

pub trait ConnectFourDataBase {
    fn new() -> ConnectFourDataBaseStruct;
    fn count_users(&mut self) -> i64;
    fn insert_user(&mut self, self_ws_id: u32, login: String) -> User;
    fn user_exists(&mut self, self_ws_id: i32) -> bool;
    fn get_user_ws_id(&mut self) -> Option<User>;
    fn get_connected_users(&mut self, self_user_id: Option<u32>) -> Option<Vec<User>>;
    fn get_user_alone(&mut self) -> Option<String>;
    fn insert_game(&mut self, id_player1: u32, id_player2: u32, grid: Grid) -> bool;
    fn play_with(&mut self, self_id_player1: u32) -> Option<GameInProgress>;
    fn update_grid(&mut self, game_id: u32, grid: &Grid) -> bool;
    fn delete_users(&mut self);
    fn delete_game_in_progress(&mut self, game_id: u32);
    fn delete_all_game_in_progress(&mut self);
    fn refresh(&mut self);
}

impl ConnectFourDataBase for ConnectFourDataBaseStruct {
    fn new() -> ConnectFourDataBaseStruct {
        ConnectFourDataBaseStruct {
            connection: establish_connection(),
            game_id:    0
        }
    }
    
    fn count_users(&mut self) -> i64 {
        match users
            .select(count(uuid))
            .filter(connected.eq(true))
            .first(&self.connection) {
             Ok(v) => v,
             Err(_) => 0
        }
    }
    
    fn insert_user(&mut self, self_ws_id: u32, self_login: String) -> User {
        let self_uuid = Uuid::new_v4().to_string();
        let user = User {
            id:        0,
            ws_id:     self_ws_id as i32,
            uuid:      self_uuid.clone(),
            admin:     false,
            points:    0,
            login:     self_login.clone(),
            passw:     None,
            connected: true,
            playing:   false
        };
        match self.user_exists(self_ws_id as i32) {
            true => {
                user
            },
            false => {
                let new_user = NewUser {
                    ws_id:     self_ws_id as i32,
                    uuid:      &self_uuid.to_string(),
                    admin:     false,
                    points:    0,
                    login:     &self_login,
                    connected: true,
                    playing:   false
                };
                insert_into(users)
                .values(&new_user)
                .execute(&self.connection).unwrap();
                user
            }
        }
    }

    fn user_exists(&mut self, self_ws_id: i32) -> bool {
        let query_fragment = users.filter(
            ws_id.eq(self_ws_id)).limit(1);
        let u = query_fragment.load::<User>(&self.connection)
        .expect("Error loading posts");
        !u.is_empty()
    }

    fn get_user_ws_id(&mut self) -> Option<User> {
        let result = users
        .filter(connected.eq(true)
        .and(playing.eq(false)))
        .limit(1)
        .load::<User>(&self.connection);
        match result {
            Ok(mut r) => {
                if r.is_empty() {
                    None
                }
                else {
                    r.pop()
                }
            },
            Err(_) => None
        }
    }

    fn get_connected_users(&mut self, self_user_id: Option<u32>) -> Option<Vec<User>> {
        let mut sql = users
        .filter(connected.eq(true).and(playing.eq(false)))
        .into_boxed();
        match self_user_id {
            Some(v) => {
                sql = users.filter(
                    connected.eq(true)
                    .and(playing.eq(false))
                    .and(ws_id.ne(v as i32))
                ).into_boxed();
            },
            None => {}
        }
        let result = sql.load::<User>(&self.connection);
        match result {
            Ok(r) => {
                if r.is_empty() {
                    None
                }
                else {
                    Some(r)
                }
            },
            Err(_) => None
        }
    }
    
    fn get_user_alone(&mut self) -> Option<String> {
        let result = users.filter(playing.eq(false))
            .limit(1)
            .load::<User>(&self.connection);
        match result {
            Ok(mut r) => {
                if r.is_empty() {
                    None
                }
                else {
                    match r.pop() {
                        Some(u) => Some(u.uuid),
                        None => None
                    }
                }
            },
            Err(_) => None
        }
    }

    fn insert_game(&mut self, self_id_player1: u32, self_id_player2: u32, grid: Grid) -> bool {
        let new_game = NewGameInProgress {
            id_player1:     self_id_player1 as i32,
            id_player2:     self_id_player2 as i32,
            serialize_grid: serde_json::to_string(&grid).unwrap()
        };
        insert_into(game_in_progress)
        .values(&new_game)
        .execute(&self.connection).unwrap();
        self.game_id += 1;
        true
    }

    fn play_with(&mut self, self_id_player1: u32) -> Option<GameInProgress> {
        let query_fragment = game_in_progress.filter(
            id_player1.eq(self_id_player1 as i32)
            .or(id_player2.eq(self_id_player1 as i32))
        ).limit(1);
        let mut g = query_fragment.load::<GameInProgress>(&self.connection).unwrap();
        if g.is_empty() {
            None
        }
        else {
            g.pop()
        }
    }

    fn update_grid(&mut self, game_id: u32, grid: &Grid) -> bool {
        update(game_in_progress.filter(
            schema::game_in_progress::id.eq(game_id as i32)
         ))
        .set(serialize_grid.eq(serde_json::to_string(&grid).unwrap()))
        .execute(&self.connection).unwrap();
        true
    }
    
    fn delete_users(&mut self) {
        let sql = users
        .filter(
            connected.eq(true)
            //.and(playing.eq(true))
        );
        delete(sql).execute(&self.connection).unwrap();
    }
    
    fn delete_game_in_progress(&mut self, game_id: u32) {
        delete(game_in_progress.filter(
            schema::game_in_progress::id.eq(game_id as i32)
        )).execute(&self.connection).unwrap();
        self.game_id -= 1;
    }
    
    fn delete_all_game_in_progress(&mut self) {
        delete(game_in_progress).execute(&self.connection).unwrap();
    }
    
    fn refresh(&mut self) {
        self.delete_users();
        self.delete_all_game_in_progress();
    }
}

