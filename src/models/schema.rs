// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        email -> Nullable<Varchar>,
        password_hash -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        surname -> Nullable<Varchar>,
        patronym -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        steam_id -> Nullable<Varchar>,
        id -> Int4,
        email_verified -> Bool,
    }
}
