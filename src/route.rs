use actix_web::{web, HttpResponse};
use serde_json;

use crate::model::*;
use crate::message::*;

pub async fn register((req, state): (web::Json<RequestRegister>, web::Data<AppState>)) -> HttpResponse {
    let result = state.db.send(req.into_inner()).await;
    let j = serde_json::to_string(&result);
    HttpResponse::Ok().body(j)
}