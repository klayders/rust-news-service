use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

use super::super::request::query::example::ExampleQueryRequest;
use super::super::response::example::News;

#[macro_use]
use log::{error, info, warn};

#[get("/news")]
pub async fn find_one(req: web::Query<ExampleQueryRequest>) -> impl Responder {
    let request = req.into_inner();

    info!("find_one: request={}", request.newsTitle);

    let response = News {
        id: 42,
        title: request.newsTitle
    };

    HttpResponse::Ok()
        .json(response)

}