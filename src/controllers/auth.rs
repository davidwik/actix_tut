use actix_web::{Responder, post, web};
use serde::Deserialize;

use crate::{AppState, db};

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().await;

    if db::user::has_with_email(&db, &data.email).await {
        return "Email exists".to_string();
    }

    format!("Sign Up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign in"
}
