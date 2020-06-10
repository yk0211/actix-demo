use actix_web::{web, HttpResponse};
use crate::model::*;
use crate::message::*;

pub async fn register((req, state): (web::Json<RequestRegister>, web::Data<AppState>)) -> HttpResponse {
    let result = state.db.send(req.into_inner()).await;

    HttpResponse::Ok().body("fdfds")
}