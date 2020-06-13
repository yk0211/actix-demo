use actix::prelude::*;
use actix::{Actor, SyncContext};
use actix_web_actors::ws;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::NaiveDateTime;
use std::time::{Duration, Instant};

use crate::schema::t_user;

pub struct DbExcutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExcutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
    pub db: Addr<DbExcutor>
}

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsExcutor {
    pub heartbeat: Instant
}

impl Actor for WsExcutor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.set_heartbeat(ctx);
    }
}

impl WsExcutor {
    pub fn new() -> Self {
        Self {heartbeat: Instant::now()}
    }

    fn set_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
    
            ctx.ping(b"");
        });       
    }
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "t_user"]
pub struct User {
    pub uuid: String,
    pub account: String,
    pub password: String,
    pub nickname: String,
    pub gender: i8,
    pub phone_number: String,
    pub head_image: String, 
    pub create_at: NaiveDateTime,
    pub last_login_at: NaiveDateTime,
}