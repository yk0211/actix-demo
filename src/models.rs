use actix::{Actor, SyncContext};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExcutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExcutor {
    type Context = SyncContext<Self>;
}
