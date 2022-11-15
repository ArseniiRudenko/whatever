use chrono::{DateTime, FixedOffset, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub first_name: Option<String>,
    pub surname: Option<String>,
    pub patronym: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub date_of_birth: Option<NaiveDate>,
    pub email_verified: bool,
    pub steam_id: Option<String>
}

#[derive(Serialize,Deserialize)]
pub struct UserSer{
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub surname: Option<String>,
    pub patronym: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub date_of_birth: Option<String>,
    pub email_verified: bool,
    pub steam_id: Option<String>
}

#[derive(Serialize,Deserialize)]
pub struct UserClaim{
    pub email: String,
    pub roles: Vec<i32>
}

#[derive(Serialize,Deserialize)]
pub struct UserLoginInfo{
    pub email: String,
    pub password: String
}

