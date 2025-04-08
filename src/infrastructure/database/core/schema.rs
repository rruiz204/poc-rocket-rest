// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        players -> Int4,
        release_date -> Date,
        gender_id -> Int4,
    }
}

diesel::table! {
    gender (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(game -> gender (gender_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    gender,
);
