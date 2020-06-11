use actix_web::{web, Responder};
use serde_json;
use log::info;

use crate::model::*;
use crate::message::*;

pub async fn register((req, state): (web::Form<RequestRegister>, web::Data<AppState>)) -> impl Responder{
    let res = state.db.send(req.into_inner()).await;

    let content = res.and_then(|content| {
        match content {
            Ok(v) => Ok(serde_json::to_string(&v)),
            Err(e) => Ok(serde_json::to_string(&e))
        }
    });

    info!("content:{:?}", content);
    content
}
