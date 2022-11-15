// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Nullable<Varchar>,
        surname -> Nullable<Varchar>,
        patronym -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
