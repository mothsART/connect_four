CREATE TABLE users (
  id        INTEGER NOT NULL PRIMARY KEY,
  ws_id     INTEGER NOT NULL,
  uuid      VARCHAR(36),
  admin     BOOLEAN NOT NULL DEFAULT 0,
  login     VARCHAR(64),
  passw     VARCHAR(64),
  points    INTEGER NOT NULL,
  connected BOOLEAN NOT NULL DEFAULT 0,
  playing   BOOLEAN NOT NULL DEFAULT 0
);
