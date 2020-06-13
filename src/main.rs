#[macro_use] 
extern crate diesel;

#[macro_use]
extern crate serde;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use config::{FileFormat, File, Config};
use log4rs;

mod model;
mod schema;
mod error;
mod route;
mod handler;
mod message;

use model::{DbExcutor, AppState};
use route::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut config = Config::default();
    config.merge(File::new("config", FileFormat::Toml)).expect("There are not find config file");
   
    let db_url = config.get_str("mysql.db_url").unwrap();
    let worker_num = config.get_int("mysql.worker_num").unwrap();
    let bind_url = config.get_str("web.bind_url").unwrap();
    let log4rs_path = config.get_str("log.log4rs_path").unwrap();
    log4rs::init_file(log4rs_path, Default::default()).unwrap();

    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    let address = SyncArbiter::start(worker_num as usize, move || DbExcutor(pool.clone()));    

    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj "/CN=www.stellar.com"
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || { 
        App::new()
            .data(AppState { db: address.clone() })
            .wrap(Logger::default())
            .default_service(web::route().to(|| HttpResponse::NotFound()))
            .service(web::resource("/ws/").route(web::get().to(websocket)))
            .service(Files::new("/public", "./public").show_files_listing().use_last_modified(true))           
            .service(web::scope("/user")
                .route("/register", web::post().to(register)))
    })
    .bind_openssl(bind_url, builder)?
    .run()
    .await
}