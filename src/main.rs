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
    repositories::user_repo::UserRepository,
    routes::create_router,
    services::{auth_service::AuthService, user_service::UserService},
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::<PgPool>::new(init_db(&db_url).await);

    let user_repo = Arc::new(UserRepository::new(db.clone()));

    let auth_service = Arc::new(AuthService::new(user_repo.clone()));
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
