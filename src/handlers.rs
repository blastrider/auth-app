use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthData {
    username: String,
    password: String,
}

pub async fn login(data: web::Json<AuthData>) -> HttpResponse {
    // Dummy authentication logic
    if data.username == "admin" && data.password == "password" {
        info!("Successful login for user: {}", data.username);
        HttpResponse::Ok().json("Login successful")
    } else {
        error!("Failed login attempt for user: {}", data.username);
        HttpResponse::Unauthorized().json("Invalid username or password")
    }
}
