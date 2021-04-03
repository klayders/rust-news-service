use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
#[macro_use]
use log::{error, info, LevelFilter, warn};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

use api::news::find_one;

mod api;
mod request;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // configure_as_json();
    configure_as_console();

    info!("booting up");

    HttpServer::new(|| App::new()
        .service(find_one)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn configure_as_console() {
    log4rs::init_file("src/resources/log4rs.yml", Default::default()).unwrap();
}

fn configure_as_json() {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}