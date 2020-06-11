use actix_web::{web, Responder};
use log::info;

use crate::model::*;
use crate::message::*;

pub async fn register((req, state): (web::Json<RequestRegister>, web::Data<AppState>)) -> impl Responder{
    let res = state.db.send(req.into_inner()).await;
    let json_res = res.and_then(|content| {
        serde::export::Ok(web::Json(content))       
    });

    info!("{:#?}", json_res);
    json_res
}
    