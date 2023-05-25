mod api;
mod models;
mod repository;

use crate::api::user_api::{create_user, delete_user, get_user, list_user, update_user};
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(list_user)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
