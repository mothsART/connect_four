CREATE TABLE game_in_progress (
  id             INTEGER NOT NULL PRIMARY KEY,
  id_player1     INTEGER NOT NULL,
  id_player2     INTEGER NOT NULL,
  serialize_grid varchar(255)
);
