use actix::prelude::*;
use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::NaiveDateTime;

use crate::schema::t_user;

pub struct DbExcutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExcutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
    pub db: Addr<DbExcutor>
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "t_user"]
pub struct User {
    pub uuid: String,
    pub gender: i8,
    pub phone_number: String,
    pub head_url: String,
    pub account: String,
    pub password: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}