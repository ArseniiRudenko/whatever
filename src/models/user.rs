use chrono::{DateTime, FixedOffset, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone)]
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub surname: Option<String>,
    pub patronym: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Serialize,Deserialize)]
pub struct UserSer{
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub surname: Option<String>,
    pub patronym: Option<String>,
    pub created_at: String,
    pub date_of_birth: Option<String>,
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

