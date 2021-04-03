use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

use super::super::request::query::example::ExampleQueryRequest;
use super::super::response::example::News;



#[get("/news")]
pub async fn find_one(info: web::Query<ExampleQueryRequest>) -> impl Responder {
    // let db = state.as_ref().db.clone();
    println!("data={}", info.newsTitle);
    let example = News {
        id: 42,
        title: info.newsTitle.clone()
    };
    HttpResponse::Ok().json(example)

}