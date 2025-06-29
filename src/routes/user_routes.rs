use axum::{Router}; // , routing::get};
// use std::sync::Arc;
// use crate::handlers::user_handler::UserHandler;
use crate::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
    //    .route("/:id", get(move |id| handler.get_user(id)))*/
}
