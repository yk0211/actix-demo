use actix_web::{web, Responder};
//use serde_json;
use log::*;

use crate::model::*;
use crate::message::*;

pub async fn register((req, state): (web::Json<RequestRegister>, web::Data<AppState>)) -> impl Responder{
    let res = state.db.send(req.into_inner()).await;
    
    let j = res.map_err(|err| { 
        error!("{:#?}", err);
    })
    .and_then(|content| {
        info!("{:#?}", content);
        Ok("ddfd")
    });

    j
}
    