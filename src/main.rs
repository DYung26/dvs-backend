use std::env;
use std::sync::Arc;

use tokio::net::TcpListener;
use dotenvy::dotenv;

pub mod schema;
pub mod state;

mod handlers;
mod db;
mod repositories;
mod routes;
mod services;
mod models;
mod dto;
mod utils;

use crate::{
    db::{init_db, PgPool},
    handlers::{auth_handler::AuthHandler, user_handler::UserHandler},
    repositories::{
        user_repo::UserRepository,
        otp_repo::OTPRepository,
    },
    routes::create_router,
    services::{
        auth_service::AuthService, user_service::UserService,
        email_service::EmailService,
    },
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::<PgPool>::new(init_db(&db_url).await);

    let api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
    let from_email = env::var("EMAIL_FROM").expect("EMAIL_FROM must be set");
    let from_name = env::var("EMAIL_FROM_NAME").expect("EMAIL_FROM_NAME must be set");

    let user_repo = Arc::new(UserRepository::new(db.clone()));
    let otp_repo = Arc::new(OTPRepository::new(db.clone()));

    let email_service = Arc::new(EmailService::new(api_key, from_email, from_name));
    let auth_service = Arc::new(AuthService::new(
            user_repo.clone(), otp_repo.clone(), email_service.clone(),
        )
    );
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    let auth_handler = Arc::new(AuthHandler::new(auth_service));
    let user_handler = Arc::new(UserHandler::new(user_service));

    let state = AppState {
        auth_handler,
        user_handler,
    };

    let app = create_router().with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
