#[macro_use] extern crate log;
extern crate time;
extern crate ws;
extern crate env_logger;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate connect_four;
extern crate uuid;

use std::fs::OpenOptions;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use serde_json::Value as Json;

use diesel::*;
use diesel::sqlite::SqliteConnection;
use connect_four::{establish_connection, ADDR, PORT};
use connect_four::grid::{win, Grid};
use connect_four::models::{User, NewUser, GameInProgress, NewGameInProgress};
use connect_four::schema::users::dsl::users;
use connect_four::schema::game_in_progress::dsl::game_in_progress;
use uuid::Uuid;

const SAVE: ws::util::Token = ws::util::Token(1);
const PING: ws::util::Token = ws::util::Token(2);
const FILE: &'static str = "message_log";
const SAVE_TIME: u64 = 500;
const PING_TIME: u64 = 10_000;

type MessageLog = Rc<RefCell<Vec<LogMessage>>>;
type Users      = Rc<RefCell<HashMap<u32, WSUser>>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wrapper {
    path: String,
    content: Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    nick: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Join {
    join_nick: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayWith {
    user_id: i32,
    nick: String,
    opponent_nick: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Agree {
	response: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
struct WSUser {
    name:    Option<String>,
    play_on: Option<i32>,
    color:   DiscColor
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Play {
    id: i32,
    disc_x: i32,
    disc_y: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
enum DiscColor {
    None   = 0,
    Yellow = 1,
    Red    = 2
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LogMessage {
    nick: String,
    sent: Option<i64>,
    message: String,
}

impl LogMessage {
    fn to_message(&self) -> Message {
        Message {
            nick: self.nick.clone(),
            message: self.message.clone(),
        }
    }
}

impl Message {
    fn into_log(self) -> LogMessage {
        LogMessage {
            nick: self.nick,
            message: self.message,
            sent: Some(time::get_time().sec), // discard nanoseconds
        }
    }
}

pub struct ConnectFourDataBaseStruct {
    pub connection: SqliteConnection
}

pub trait ConnectFourDataBase {
    fn new() -> ConnectFourDataBaseStruct;
    fn count_users(&mut self) -> i64;
    fn insert_user(&mut self, ws_id: u32, login: String) -> User;
    fn user_exists(&mut self, ws_id_to_test: i32) -> bool;
    fn get_user_ws_id(&mut self) -> Option<User>;
    fn get_connected_users(&mut self) -> Option<Vec<User>>;
    fn get_user_alone(&mut self) -> Option<String>;
    fn insert_game(&mut self, id_player1: u32, id_player2: u32, grid: Grid) -> bool;
    fn play_with(&mut self, id_player1: u32) -> Option<GameInProgress>;
    fn update_grid(&mut self, id_player1: u32, grid: &Grid) -> bool;
}

impl ConnectFourDataBase for ConnectFourDataBaseStruct {
    fn new() -> ConnectFourDataBaseStruct {
        ConnectFourDataBaseStruct {
            connection:  establish_connection()
        }
    }

    fn count_users(&mut self) -> i64 {
        use diesel::dsl::*;
        use connect_four::schema::users::*;
        match connect_four::schema::users::dsl::users
            .select(count(uuid))
            .filter(connected.eq(true))
            .first(&self.connection) {
             Ok(v) => v,
             Err(_) => 0
        }
    }
    
    fn insert_user(&mut self, ws_id: u32, login: String) -> User {
        let uuid = Uuid::new_v4().to_string();
		let user = User {
			id:        0,
			ws_id:     ws_id as i32,
			uuid:      uuid.clone(),
			admin:     false,
			points:    0,
			login:     login.clone(),
			passw:     None,
			connected: true,
			playing:   false
		};
        match self.user_exists(ws_id as i32) {
            true => {
                user
            },
            false => {
                let new_user = NewUser {
                    ws_id:     ws_id as i32,
                    uuid:      &uuid.to_string(),
                    admin:     false,
                    points:    0,
                    login:     &login,
                    connected: true,
                    playing:   false
                };
                diesel::insert_into(users)
                .values(&new_user)
                .execute(&self.connection);
                user
            }
        }
    }

    fn user_exists(&mut self, ws_id_to_test: i32) -> bool {
        use connect_four::schema::users::*;
        let query_fragment = connect_four::schema::users::dsl::users.filter(
            ws_id.eq(ws_id_to_test)).limit(1);
        let u = query_fragment.load::<User>(&self.connection)
        .expect("Error loading posts");
        !u.is_empty()
    }

    fn get_user_ws_id(&mut self) -> Option<User> {
        use connect_four::schema::users::*;
        let result = connect_four::schema::users::dsl::users
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

    fn get_connected_users(&mut self) -> Option<Vec<User>> {
        use connect_four::schema::users::*;
        let result = connect_four::schema::users::dsl::users
        .filter(connected.eq(true)
        .and(playing.eq(false)))
        .load::<User>(&self.connection);
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
        use connect_four::schema::users::*;
        let result = connect_four::schema::users::dsl::users.
        filter(playing.eq(false))
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

    fn insert_game(&mut self, id_player1: u32, id_player2: u32, grid: Grid) -> bool {
        let new_game = NewGameInProgress {
            id_player1:     id_player1 as i32,
            id_player2:     id_player2 as i32,
            serialize_grid: serde_json::to_string(&grid).unwrap()
        };
        diesel::insert_into(game_in_progress)
        .values(&new_game);
        true
    }
    
    fn play_with(&mut self, id_player1: u32) -> Option<GameInProgress> {
        use connect_four::schema::game_in_progress::*;
        let query_fragment = connect_four::schema::game_in_progress::dsl::game_in_progress.filter(
            id_player1.eq(id_player1)
            .or(id_player2.eq(id_player1))
        ).limit(1);
        let mut g = query_fragment.load::<GameInProgress>(&self.connection).unwrap();
        if g.is_empty() {
            None
        }
        else {
            g.pop()
        }
    }

    fn update_grid(&mut self, id_player2: u32, grid: &Grid) -> bool {
        use connect_four::schema::game_in_progress::*;
        diesel::update(connect_four::schema::game_in_progress::dsl::game_in_progress.filter(
            id_player2.eq(id_player2)
         ))
        .set(serialize_grid.eq(serde_json::to_string(&grid).unwrap()))
        .execute(&self.connection);
        true
    }
}

struct ChatHandler {
    out: ws::Sender,
    db: ConnectFourDataBaseStruct,
    nick: Option<String>,
    message_log: MessageLog,
    users: Users,
    not_playing_uuid:  Option<RefCell<String>>
}

use ws::{Request, Result, Response};
// https://github.com/housleyjk/ws-site-chat/blob/master/src/main.rs
impl ws::Handler for ChatHandler {

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        try!(self.out.timeout(SAVE_TIME, SAVE));
        try!(self.out.timeout(PING_TIME, PING));
        let backlog = self.message_log.borrow();
        // We take two chunks because one chunk might not be a full 50
        /*let mut it = backlog.chunks(50).rev().take(2);
        let msgs1 = it.next();
        let msgs2 = it.next();
        // longwinded reverse
        println!("cool!");
        if let Some(msgs) = msgs2 {
            for msg in msgs {
                if let Some(sent) = msg.sent {
                    if time::get_time() - time::Timespec::new(sent, 0) < time::Duration::minutes(10) {
                        try!(self.out.send(format!("{:?}", json!({
                            "path": "/message",
                            "content": msg.to_message(),
                            "users_nb": self.db.count_users()
                        }))))
                    }
                }
            }
        }
        if let Some(msgs) = msgs1 {
            for msg in msgs {
                if let Some(sent) = msg.sent {
                    if  time::get_time() - time::Timespec::new(sent, 0) < time::Duration::minutes(10) {
                        try!(self.out.send(format!("{:?}", json!({
                            "path": "/message",
                            "content": msg.to_message(),
                        }))))
                    }
                }
            }
        }*/
        Ok(())
    }

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        let mut res = try!(Response::from_request(req));
        if try!(req.extensions()).iter().find(|&&ext| ext.contains("myextension-name")).is_some() {
            res.add_extension("myextension-name")
        }
        Ok(res)
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let id = self.out.connection_id();
        if let Ok(text_msg) = msg.clone().as_text() {
            //println!("debug {:?}", text_msg);
            if let Ok(wrapper) = serde_json::from_str::<Wrapper>(text_msg) {
                 if wrapper.path == "connected" {
                    return self.out.send(format!("{}",
                        json!({
                            "path": "connected",
                            "user_id": id,
                            "users_nb": self.db.count_users()
                        })
                    ));
                }
                if wrapper.path == "user_list" {
                    return self.out.send(format!("{}",
                        json!({
                            "path": "user_list",
                            "users": serde_json::to_value(self.db.get_connected_users()).unwrap()
                        })
                    ))
                }
                if let Ok(join) = serde_json::from_value::<Join>(wrapper.content.clone()) {
                    let join_nick = join.join_nick.clone();
                    let user = self.db.insert_user(id, join_nick.clone());
                    return self.out.broadcast(format!("{}",
                        json!({
                            "path": "has_joined",
                            "user": serde_json::to_value(user).unwrap()
                        })
                    ))
                }
                if let Ok(play_with) = serde_json::from_value::<PlayWith>(wrapper.content.clone()) {
                    self.out.send_to(play_with.user_id as u32, format!("{}",
                        json!({
                            "path": "agree",
                            "opponent_nick": play_with.nick.clone(),
                            "opponent_id": id
                        })
                    )).unwrap();
                    return self.out.send(format!("{}",
                        json!({
                            "path": "agree",
                            "opponent": play_with.opponent_nick,
                            "begin": false
                        })
                    ))
                }
                /*
                if let Ok(agree) = serde_json::from_value::<Agree>(wrapper.content.clone()) {
                    /* Create a game and start it ! */
                    let grid = Grid::new();
                    self.db.insert_game(id, play_with.user_id as u32, grid);
                    
                    self.out.send_to(play_with.user_id as u32, format!("{}",
                        json!({
                            "path": "game_start",
                            "opponent": play_with.nick.clone(),
                            "begin": true,
                            "color": DiscColor::Yellow,
                            "opponent_color": DiscColor::Red
                        })
                    )).unwrap();
                    return self.out.send(format!("{}",
                        json!({
                            "path": "game_start",
                            "opponent": play_with.opponent_nick,
                            "begin": false,
                            "color": DiscColor::Red,
                            "opponent_color": DiscColor::Yellow
                        })
                    ))
                }
                */
                /*if let Ok(join) = serde_json::from_value::<Join>(wrapper.content.clone()) {
                    // first : give the alone user id
                    let user_alone = self.db.get_user_ws_id();
                    let join_nick = join.join_nick.clone();
                    let uuid = self.db.insert_user(id, join_nick.clone());
                    match user_alone {
                        Some(second_user) => {
                            /* Create a game and start it ! */
                            let grid = Grid::new();
                            self.db.insert_game(id, second_user.ws_id as u32, grid);
                            let mut users_m  = self.users.borrow_mut();
                            users_m.insert(id, WSUser {
                                name: Some(join_nick.clone()),
                                play_on: Some(second_user.id as i32),
                                color: DiscColor::Red
                            });
                            /*users_m[&(second_user.id as u32)] = WSUser {
                                name: second_user.login.clone(),
                                play_on: Some(id as i32),
                                color: DiscColor::Red
                            };*/
                            println!("exist! second user id : {}", second_user.ws_id);
                            self.out.send_to(second_user.ws_id as u32, format!("{}",
                                json!({
                                    "path": "game_start",
                                    "opponent": join_nick.clone(),
                                    "begin": true,
                                    "color": DiscColor::Yellow,
                                    "opponent_color": DiscColor::Red
                                })
                            )).unwrap();
                            return self.out.send(format!("{}",
                                json!({
                                    "path": "game_start",
                                    "opponent": second_user.login,
                                    "begin": false,
                                    "color": DiscColor::Red,
                                    "opponent_color": DiscColor::Yellow
                                })
                            ))
                        }
                        None => {
                            /* Keep user in waiting */
                            self.users.borrow_mut().insert(id, WSUser {
                                name:    Some(join.join_nick),
                                play_on: None,
                                color:   DiscColor::Yellow
                            });
                            return self.out.send(format!("{}",
                                json!({
                                    "path": "wait",
                                    "message": "Please wait : nobody to play actualy."
                                })
                            ));
                        }
                    }
                }*/
                if let Ok(play) = serde_json::from_value::<Play>(wrapper.content.clone()) {
                    let game = self.db.play_with(id).unwrap();
                    let color_int = self.users.borrow().get(&id).unwrap().color.clone() as i8;
                    let mut grid: Grid = serde_json::from_str(&game.serialize_grid.unwrap()).unwrap();
                    grid.update(play.disc_x as usize, play.disc_y as usize, color_int);
                    let mut second_player_id = game.id_player1;
                    if id == game.id_player1 as u32 {
                        second_player_id = game.id_player2;
                    }
                    self.db.update_grid(second_player_id as u32, &grid);
                    match win(
                        &grid.grid,
                        play.disc_x as usize,
                        play.disc_y as usize, 
                        color_int
                    ) {
                        true => {
                            self.out.send_to(second_player_id as u32, format!("{}",
                                json!({
                                    "path": "game_over",
                                    "x": play.disc_x
                                    
                                })
                            )).unwrap();
                            return self.out.send(format!("{}",
                                json!({
                                    "path": "win"
                                })
                            ))
                        },
                        false => {
                            self.out.send_to(second_player_id as u32, format!("{}",
                                json!({
                                    "path": "play",
                                    "x": play.disc_x
                                })
                            )).unwrap();
                            return self.out.send(format!("{}",
                                json!({
                                    "path": "has_played"
                                })
                            ))
                        }
                    }
                }
            }
        }
        self.out.send(format!("{}", json!({
            "path": "/error",
            "content": format!("Unable to parse message ! {}", msg),
        })))
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        if let Some(nick) = self.nick.as_ref() {
            self.users.borrow_mut().remove(&self.out.connection_id());
            let leave_msg = Message {
                nick: "system".into(),
                message: format!("{} has left the chat.", nick),
            };
            self.message_log.borrow_mut().push(leave_msg.clone().clone().into_log());
            if let Err(err) = self.out.broadcast(format!("{}", json!({
                "path": "/left",
                "content": leave_msg,
            }))) {
                error!("{:?}", err);
            }
        }
    }

    fn on_timeout(&mut self, tok: ws::util::Token) -> ws::Result<()> {
        match tok {
            SAVE => {
                let mut file = try!(OpenOptions::new().write(true).open(FILE));
                if let Err(err) = serde_json::to_writer_pretty::<_, Vec<LogMessage>>(
                    &mut file,
                    self.message_log.borrow().as_ref())
                {
                   Ok(error!("{:?}", err))
                } else {
                    self.out.timeout(SAVE_TIME, SAVE)
                }
            }
            PING => {
                try!(self.out.ping(Vec::new()));
                self.out.timeout(PING_TIME, PING)
            }
            _ => unreachable!()
        }
    }
}

fn main () {
    // Setup logging
    env_logger::init().unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE)
        .expect("Unable to open message log.");

    let message_log = MessageLog::new(
        RefCell::new(
            serde_json::from_reader(
                &mut file,
            ).unwrap_or(
                Vec::with_capacity(10_000))));

    let _users = Users::new(RefCell::new(HashMap::with_capacity(10_000)));
    if let Err(error) = ws::listen(format!("{}:{}", ADDR, PORT), |out| {
        ChatHandler {
            out: out,
            db: ConnectFourDataBaseStruct::new(),
            nick: None,
            message_log: message_log.clone(),
            users: _users.clone(),
            not_playing_uuid: None
        }
    }) {
        error!("Failed to create WebSocket due to {:?}", error);
    }
}
