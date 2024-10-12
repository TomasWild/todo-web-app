use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use crate::{
    api::model::todo::{NewTodo, Todo, UpdateTodo},
    error::Error,
    utils::app_state::AppState,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(new_todo): Json<NewTodo>,
) -> Result<impl IntoResponse, Error> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todo (id, title, description, is_done, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, title, description, is_done, created_at, updated_at
        "#,
    )
        .bind(Uuid::new_v4())
        .bind(&new_todo.title)
        .bind(&new_todo.description)
        .bind(new_todo.is_done)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&state.pool)
        .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_all_todos(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, description, is_done, created_at, updated_at
        FROM todo
        "#
    )
        .fetch_all(&state.pool)
        .await?;

    Ok((StatusCode::OK, Json(todos)))
}

pub async fn find_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, description, is_done, created_at, updated_at
        FROM todo
        WHERE id = $1
        "#
    )
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;

    match todo {
        Some(todo) => Ok((StatusCode::OK, Json(todo))),
        None => Err(Error::NotFound)
    }
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, Error> {
    let result = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todo
        SET
            title = $2,
            description = $3,
            is_done = $4
        WHERE id = $1
        RETURNING id, title, description, is_done, created_at, updated_at
        "#
    )
        .bind(id)
        .bind(&update_todo.title)
        .bind(&update_todo.description)
        .bind(update_todo.is_done)
        .fetch_optional(&state.pool)
        .await?;

    match result {
        Some(updated_todo) => Ok((StatusCode::OK, Json(updated_todo))),
        None => Err(Error::NotFound),
    }
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let result = sqlx::query("DELETE FROM todo WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}