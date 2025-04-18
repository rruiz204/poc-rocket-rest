// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        players -> Int4,
        release_date -> Date,
        game_gender_id -> Int4,
    }
}

diesel::table! {
    gamegender (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(game -> gamegender (game_gender_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    gamegender,
);
