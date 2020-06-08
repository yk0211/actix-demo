
use actix::prelude::*;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};

use diesel::{r2d2::ConnectionManager, MysqlConnection};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use config::{FileFormat, File, Config};

mod models;
use models::DbExcutor;
#[allow(dead_code)]
struct AppState {
    db: Addr<DbExcutor>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // create a self-signed temporary cert for testing:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj "/C=CN/ST=sh/L=sh/O=stellar/OU=IT/CN=localhost"
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let mut config = Config::default();
    config.merge(File::new("config", FileFormat::Toml)).unwrap();
   
    let db_url = config.get_str("mysql.db_url").unwrap();
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    let address = SyncArbiter::start(4, move || DbExcutor(pool.clone())); 

    HttpServer::new(move || { 
        App::new()
            .data(AppState { db: address.clone() })
            .wrap(Logger::default())
            .service(Files::new("/static", "./public").show_files_listing().use_last_modified(true))
            .service(web::scope("/users").route("/show", web::get().to(index)))
    })
    .bind_openssl("127.0.0.1:8088", builder)?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("test....")
}