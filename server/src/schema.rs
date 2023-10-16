// @generated automatically by Diesel CLI.

diesel::table! {
    items (item_id) {
        item_id -> Int4,
        #[max_length = 20]
        item_name -> Varchar,
        item_type -> Int4,
        price -> Int4,
        is_active -> Bool,
        update_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 15]
        account -> Varchar,
        #[max_length = 20]
        password -> Varchar,
        #[max_length = 15]
        nickname -> Nullable<Varchar>,
        coin -> Int4,
        #[max_length = 40]
        refresh_token -> Nullable<Varchar>,
        last_login_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    items,
    users,
);
