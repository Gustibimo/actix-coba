mod models;

use actix_web::{HttpServer, App, web, Responder};
use std::io;
use crate::models::Status;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
}

async fn index() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status:"Down".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    println!("Starting at port:8000");
    HttpServer::new(|| {
        App::new()
            .route("/status", web::get().to(status))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
