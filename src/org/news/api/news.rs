use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
extern crate redis;
use redis::Commands;

use super::super::request::query::example::ExampleQueryRequest;
use super::super::response::example::News;

#[macro_use]
use log::{error, info, warn};

#[get("/news")]
pub async fn find_one(req: web::Query<ExampleQueryRequest>) -> impl Responder {
    let request = req.into_inner();

    info!("find_one: request={}", request.title);

    let client = redis::Client::open("redis://127.0.0.1/");
    let mut con = client.unwrap().get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    let i :String = con.set(String::from("my_key"), 55).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let i :u32 = con.get("my_key").unwrap();

    let response = News {
        id: i,
        title: request.title
    };

    HttpResponse::Ok()
        .json(response)

}