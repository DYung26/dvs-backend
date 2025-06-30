// @generated automatically by Diesel CLI.

diesel::table! {
    otps (id) {
        id -> Uuid,
        user_id -> Uuid,
        email -> Varchar,
        #[max_length = 6]
        otp -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(otps -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    otps,
    users,
);
