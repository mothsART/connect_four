use super::schema::{users, game_in_progress};

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id:        i32,
    pub ws_id:     i32,
    pub uuid:      String,
    pub admin:     bool,
    pub login:     String,
    pub passw:     Option<String>,
    pub points:    i32,
    pub connected: bool,
    pub playing:   bool
}

#[derive(Insertable, Serialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub ws_id:     i32,
    pub uuid:      &'a str,
    pub admin:     bool,
    pub login:     &'a str,
    pub points:    i32,
    pub connected: bool,
    pub playing:   bool
}

#[derive(Queryable)]
pub struct PlayWith  {
    pub id:        i32,
    pub id_vs:     i32,
    pub nb_of:     i32,
    pub win:       i32
}

#[derive(Debug, Queryable)]
pub struct GameInProgress {
    pub id:             i32,
    pub id_player1:     i32,
    pub id_player2:     i32,
    pub serialize_grid: Option<String>
}

#[derive(Insertable)]
#[table_name="game_in_progress"]
pub struct NewGameInProgress {
    pub id_player1:     i32,
    pub id_player2:     i32,
    pub serialize_grid: String
}
