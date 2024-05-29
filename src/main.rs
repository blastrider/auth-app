use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use fern::Dispatch;
use log::LevelFilter;
use std::env;
use std::fs::OpenOptions;

mod handlers;
mod models;
mod routes;
mod utils;

fn setup_logger() -> Result<(), fern::InitError> {
    let log_file_path = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "logs/log.txt".to_string());
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_file_path)?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(log_file)
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logger().expect("Failed to initialize logger");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| App::new().configure(routes::init))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
