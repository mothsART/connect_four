#[macro_use] extern crate log;
extern crate time;
extern crate ws;
extern crate env_logger;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate connect_four;

use std::fs::OpenOptions;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use serde_json::Value as Json;
use connect_four::{
    ADDR, PORT,
    ConnectFourDataBaseStruct, ConnectFourDataBase
};
use connect_four::grid::{Grid, win};

const SAVE: ws::util::Token = ws::util::Token(1);
const PING: ws::util::Token = ws::util::Token(2);
const FILE: &'static str = "message_log";
const SAVE_TIME: u64 = 500;
const PING_TIME: u64 = 10_000;

type MessageLog = Rc<RefCell<Vec<LogMessage>>>;
type Users      = Rc<RefCell<HashMap<u32, ws::Sender>>>;
type GameIP = Rc<RefCell<GamesInProgres>>;

struct GamesInProgres {
    length: u32
}

impl GamesInProgres {
    fn new() -> GamesInProgres {
        GamesInProgres {
            length: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wrapper {
    path:    String,
    content: Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    nick:    String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Join {
    join_nick: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayWithRandomUser {
    nick:          String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayWith {
    user_id:       i32,
    nick:          String,
    opponent_nick: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Agree {
    opponent_id:   i32,
    nick:          String,
    opponent_nick: String,
    response:      bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
struct WSUser {
    name:    Option<String>,
    play_on: Option<i32>,
    color:   DiscColor
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Play {
    game_id: u32,
    user_id: i32,
    disc_x:  i32,
    disc_y:  i32,
    color:   DiscColor
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
enum DiscColor {
    None   = 0,
    Yellow = 1,
    Red    = 2
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LogMessage {
    nick:    String,
    sent:    Option<i64>,
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

struct ChatHandler {
    out: ws::Sender,
    db: ConnectFourDataBaseStruct,
    nick: Option<String>,
    message_log: MessageLog,
    users: Users,
    games_in_p: GameIP
}

use ws::{Request, Result, Response};
// https://github.com/housleyjk/ws-site-chat/blob/master/src/main.rs
impl ws::Handler for ChatHandler {

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        try!(self.out.timeout(SAVE_TIME, SAVE));
        try!(self.out.timeout(PING_TIME, PING));
        let backlog = self.message_log.borrow();
        // We take two chunks because one chunk might not be a full 50
        let mut it = backlog.chunks(50).rev().take(2);
        let msgs1 = it.next();
        let msgs2 = it.next();
        // longwinded reverse
        if let Some(msgs) = msgs2 {
            for msg in msgs {
                if let Some(sent) = msg.sent {
                    if time::get_time() - time::Timespec::new(sent, 0) < time::Duration::minutes(10) {
                        try!(self.out.send(format!("{:?}", json!({
                            "path":     "/message",
                            "content":  msg.to_message(),
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
                            "path":    "/message",
                            "content": msg.to_message(),
                        }))))
                    }
                }
            }
        }
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
            if let Ok(wrapper) = serde_json::from_str::<Wrapper>(text_msg) {
                self.users.borrow_mut().insert(self.out.connection_id(), self.out.clone());
                 if wrapper.path == "connected" {
                    return self.out.send(format!("{}",
                        json!({
                            "path":     "connected",
                            "user_id":  id,
                            "users_nb": self.db.count_users()
                        })
                    ))
                }
                if wrapper.path == "user_list" {
                    return self.out.send(format!("{}",
                        json!({
                            "path":  "user_list",
                            "users": serde_json::to_value(self.db.get_connected_users(Some(id))).unwrap()
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
                if wrapper.path == "random_opponent" {
                    println!("{:?}", wrapper.content);
                    if let Ok(play_with) = serde_json::from_value::<PlayWithRandomUser>(wrapper.content.clone()) {
                        match self.db.get_random_user(id) {
                            Some(u) => {
                                let mut users = self.users.borrow_mut();
                                users.get(&(u.ws_id as u32)).unwrap().send(format!("{}",
                                    json!({
                                        "path": "game_request",
                                        "opponent_nick": play_with.nick,
                                        "opponent_id":   id
                                    })
                                )).unwrap();
                                return self.out.send(format!("{}",
                                    json!({
                                        "path":          "wait_agreement",
                                        "opponent_nick": u.login,
                                        "opponent_id":   u.id
                                    })
                                ))
                            },
                            None => {
                                return self.out.send(format!("{}",
                                    json!({
                                        "path":          "No_users"
                                    })
                                ))
                           }
                        }
                    }
                }
                if wrapper.path == "play_with" {
                    if let Ok(play_with) = serde_json::from_value::<PlayWith>(wrapper.content.clone()) {
                        println!("play _with ---> {:?}", play_with.user_id as u32);
                        let mut users = self.users.borrow_mut();
                        match users.contains_key(&(play_with.user_id as u32)) {
                            true => {
                                users.get(&(play_with.user_id as u32)).unwrap().send(format!("{}",
                                    json!({
                                        "path": "game_request",
                                        "opponent_nick": play_with.nick.clone(),
                                        "opponent_id":   id
                                    })
                                )).unwrap();
                                return self.out.send(format!("{}",
                                    json!({
                                        "path":          "wait_agreement",
                                        "opponent_nick": play_with.opponent_nick,
                                        "opponent_id":   play_with.user_id
                                    })
                                ))
                            },
                            false => {
                              return self.out.send(format!("{}",
                                  json!({
                                      "path":          "lost_user",
                                      "opponent_nick": play_with.opponent_nick
                                  })
                              ))
                            }
                        }
                    }
                }
                if wrapper.path == "agree" {
                    if let Ok(agree) = serde_json::from_value::<Agree>(wrapper.content.clone()) {
                        println!("agree {:?}", agree);
                        match agree.response {
                            true => {
                                /* Create a game and start it ! */
                                let grid = Grid::new();
                                self.db.insert_game(id, agree.opponent_id as u32, grid);
                                self.games_in_p.borrow_mut().length += 1;
                                return self.out.broadcast(format!("{}",
                                    json!({
                                        "path":    "game_start",
                                        "game_id": self.games_in_p.borrow_mut().length,
                                        "user": {
                                            "id":      id,
                                            "nick":    agree.nick.clone(),
                                            "color":   DiscColor::Red
                                        },
                                        "opponent": {
                                            "id":      agree.opponent_id,
                                            "nick":    agree.opponent_nick,
                                            "color":   DiscColor::Yellow
                                        }
                                    })
                                ))
                            },
                            false => {
                                let mut users = self.users.borrow_mut();
                                match users.contains_key(&(agree.opponent_id as u32)) {
                                    true => {
                                        return users.get(&(agree.opponent_id as u32)).unwrap().send(
                                            format!("{}",
                                                json!({
                                                    "path":          "game_refuse",
                                                    "opponent_id":   id,
                                                    "opponent_nick": agree.nick.clone()
                                                })
                                            )
                                        )
                                    },
                                    false => {
                                        return self.out.send(format!("{}",
                                            json!({
                                                "path":          "lost_user",
                                                "opponent_nick": agree.nick.clone()
                                            })
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
                if let Ok(play) = serde_json::from_value::<Play>(wrapper.content.clone()) {
                    let game = self.db.play_with(id).unwrap();
                    let mut grid: Grid = serde_json::from_str(&game.serialize_grid.unwrap()).unwrap();
                    grid.update(play.disc_x as usize, play.disc_y as usize, play.color.clone() as i8);
                    let mut second_player_id = game.id_player1;
                    if id == game.id_player1 as u32 {
                        second_player_id = game.id_player2;
                    }
                    match win(
                        &grid.grid,
                        play.disc_x as usize,
                        play.disc_y as usize, 
                        play.color.clone() as i8
                    ) {
                        true => {
                            println!("fin du jeu!");
                            self.db.delete_game_in_progress(play.game_id);
                            self.games_in_p.borrow_mut().length -= 1;
                            self.users.borrow_mut().get(&(second_player_id as u32)).unwrap().send(
                                format!("{}",
                                    json!({
                                        "path": "game_over",
                                        "x":    play.disc_x
                                        
                                    })
                                )
                            ).unwrap();
                            return self.out.send(format!("{}",
                                json!({
                                    "path": "win"
                                })
                            ))
                        },
                        false => {
                            self.db.update_grid(play.game_id, &grid);
                            self.users.borrow_mut().get(&(second_player_id as u32)).unwrap().send(
                                format!("{}",
                                    json!({
                                        "path": "play",
                                        "x": play.disc_x
                                    })
                                )
                            ).unwrap();
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
                Vec::with_capacity(10_000)
            )
        )
    );
    let _users = Users::new(RefCell::new(HashMap::with_capacity(10_000)));
    if cfg!(debug_assertions) {
        let mut db = ConnectFourDataBaseStruct::new();
        println!("DEV MODE : refresh database");
        db.refresh();
    }
    let _games_in_p = GameIP::new(RefCell::new(GamesInProgres::new()));
    if let Err(error) = ws::listen(format!("{}:{}", ADDR, PORT), |out| {
        ChatHandler {
            out:         out,
            db:          ConnectFourDataBaseStruct::new(),
            nick:        None,
            message_log: message_log.clone(),
            users:       _users.clone(),
            games_in_p:  _games_in_p.clone()
        }
    }) {
        error!("Failed to create WebSocket due to {:?}", error);
    }
}
