use actix_web::{web, Responder, HttpRequest};
use actix_web_actors::ws;
use serde_json;
use log::info;

use crate::model::*;
use crate::message::*;

pub async fn websocket(req: HttpRequest, stream: web::Payload, _: web::Data<AppState>) -> impl Responder {
    let res = ws::start(WsExcutor::new(), &req, stream);

    info!("res:{:?}", res);
    res
}

pub async fn register(req: web::Form<RequestRegister>, state: web::Data<AppState>) -> impl Responder{
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
