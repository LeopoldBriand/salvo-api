use salvo::prelude::Extractible;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(FromRow, Serialize, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body"))]
pub struct SigninUserForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body"))]
pub struct LoginUserForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uuid: String,
    pub exp: i64,
}