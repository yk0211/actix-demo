use actix::{Handler, Message};
use chrono::{Duration, Local};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::error::ServiceError;
use crate::model::{DbExcutor, User};
use crate::message::*;
use crate::schema::t_user::dsl::{t_user, account};

impl Message for RequestRegister {
    type Result = Result<ResponseRegister, ResponseError>;
}

impl Handler<RequestRegister> for DbExcutor {
    type Result = Result<ResponseRegister, ResponseError>;

    fn handle(&mut self, msg: RequestRegister, _: &mut Self::Context) -> Self::Result {      
        let conn: &MysqlConnection = &self.0.get().unwrap();

        let results = t_user
        .filter(account.eq(&msg.account))
        .limit(1)
        .load::<User>(conn)
        .map_err(|_| {
            ResponseError { code: ServiceError::InternalServerError as u32}
        })?;

        if results.len() > 0 {
            return Err(ResponseError { code: ServiceError::AccountHasExist as u32});
        }

        let new_user = User {
            uuid: Uuid::new_v4().to_string(),
            gender: 1,
            phone_number: msg.phone_number,
            head_url: String::from(""),
            account: msg.account,
            password: msg.password,
            create_at: Local::now().naive_local(),
            update_at: Local::now().naive_local() + Duration::hours(24)
        };

        diesel::insert_into(t_user)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| {
            ResponseError { code: ServiceError::InternalServerError as u32}
        })?;   

        let resp = ResponseRegister {
            code: ServiceError::Successful as u32,
            account: new_user.account,
            password: new_user.phone_number,
        };

        Ok(resp)
    }
}