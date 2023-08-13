// @generated automatically by Diesel CLI.

diesel::table! {
    todo_item (id) {
        id -> Integer,
        #[max_length = 45]
        title -> Varchar,
        description -> Nullable<Text>,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 45]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 45]
        email -> Varchar,
    }
}

diesel::joinable!(todo_item -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_item,
    user,
);
