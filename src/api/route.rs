use crate::{
    api::handler::{
        health_check::health_check,
        todo_handler::{create_todo, delete_todo, find_all_todos, find_todo_by_id, update_todo},
    },
    db::AppState,
};
use axum::{routing::{get, post}, Router};

/// Build the Axum router with all routes
pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/api/v1/todos", post(create_todo).get(find_all_todos))
        .route("/api/v1/todos/{id}", get(find_todo_by_id).put(update_todo).delete(delete_todo))
        .with_state(app_state)
}