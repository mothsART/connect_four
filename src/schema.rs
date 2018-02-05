table! {
    users (id) {
        id -> Integer,
        ws_id -> Integer,
        uuid -> Text,
        admin -> Bool,
        login -> Text,
        passw -> Nullable<Text>,
        points -> Integer,
        connected -> Bool,
        playing -> Bool,
    }
}
table! {
    game_in_progress (id) {
        id -> Integer,
        id_player1 -> Integer,
        id_player2 -> Integer,
        serialize_grid ->  Nullable<Text>,
    }
}
