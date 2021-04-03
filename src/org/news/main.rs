use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

use api::news::find_one;

mod api;
mod request;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(find_one)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}