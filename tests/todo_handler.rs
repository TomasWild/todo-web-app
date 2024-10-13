use axum::{
    body,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::from_slice;
use sqlx::PgPool;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use todo_web_app::{
    api::{
        handler::todo_handler::{create_todo, delete_todo, find_all_todos, find_todo_by_id, update_todo},
        model::todo::{NewTodo, Todo, UpdateTodo},
    },
    db::AppState,
};
use uuid::Uuid;

#[tokio::test]
async fn test_create_todo() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    let pool = PgPool::connect(&url).await.unwrap();

    let state = AppState { pool: pool.clone() };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let new_todo = NewTodo {
        title: String::from("Test Todo"),
        description: String::from("This is a description test"),
        is_done: false,
    };

    let response = create_todo(State(state.clone()), Json(new_todo))
        .await
        .unwrap()
        .into_response();

    let status = response.status();

    assert_eq!(status, StatusCode::CREATED);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let todo: Todo = from_slice(&body_bytes).unwrap();

    assert_eq!(todo.title, "Test Todo");
    assert_eq!(todo.description, "This is a description test");
    assert!(!todo.is_done);
}

#[tokio::test]
async fn test_find_all_todos() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    let pool = PgPool::connect(&url).await.unwrap();

    let state = AppState { pool: pool.clone() };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let todos = vec![
        NewTodo {
            title: String::from("Test Todo 1"),
            description: String::from("First test todo"),
            is_done: false,
        },
        NewTodo {
            title: String::from("Test Todo 2"),
            description: String::from(""),
            is_done: true,
        },
    ];

    for todo in &todos {
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todo (title, description, is_done, created_at, updated_at)
            VALUES ($1, $2, false, NOW(), NOW())
            "#,
        )
            .bind(&todo.title)
            .bind(&todo.description)
            .fetch_all(&pool)
            .await
            .unwrap();
    }

    let response = find_all_todos(State(state.clone()))
        .await
        .unwrap()
        .into_response();

    let status = response.status();

    assert_eq!(status, StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let fetched_todos: Vec<Todo> = from_slice(&body_bytes).unwrap();

    let fetched_titles: Vec<_> = fetched_todos.iter().map(|t| &t.title).collect();
    let expected_titles: Vec<_> = todos.iter().map(|t| &t.title).collect();

    assert_eq!(fetched_titles, expected_titles);
}

#[tokio::test]
async fn test_find_todo_by_id() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    let pool = PgPool::connect(&url).await.unwrap();

    let state = AppState { pool: pool.clone() };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let title = String::from("Sample Todo");
    let description = String::from("A sample todo item for testing");

    let inserted_todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todo (title, description, is_done, created_at, updated_at)
        VALUES ($1, $2, false, NOW(), NOW())
        RETURNING id, title, description, is_done, created_at, updated_at
        "#
    )
        .bind(title)
        .bind(description)
        .fetch_one(&pool)
        .await
        .unwrap();

    let response = find_todo_by_id(State(state.clone()), Path(inserted_todo.id))
        .await
        .unwrap()
        .into_response();

    let status = response.status();

    assert_eq!(status, StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let fetched_todo: Todo = from_slice(&body_bytes).unwrap();

    assert_eq!(fetched_todo.id, inserted_todo.id);
    assert_eq!(fetched_todo.title, inserted_todo.title);
    assert_eq!(fetched_todo.description, inserted_todo.description);

    let non_existent_id = Uuid::new_v4();
    let response = find_todo_by_id(State(state.clone()), Path(non_existent_id))
        .await;

    assert!(response.is_err());
}

#[tokio::test]
async fn test_update_todo() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    let pool = PgPool::connect(&url).await.unwrap();

    let state = AppState { pool: pool.clone() };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let inserted_todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todo (title, description, is_done, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING id, title, description, is_done, created_at, updated_at
        "#,
    )
        .bind("Initial Title".to_string())
        .bind("Initial Description".to_string())
        .bind(false)
        .fetch_one(&pool)
        .await
        .unwrap();

    let updated_todo = UpdateTodo {
        title: Option::from("Updated Title".to_string()),
        description: Some("Updated Description".to_string()),
        is_done: Option::from(true),
    };

    let response = update_todo(
        State(state.clone()),
        Path(inserted_todo.id),
        Json(updated_todo.clone()),
    )
        .await
        .unwrap()
        .into_response();

    let status = response.status();

    assert_eq!(status, StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: UpdateTodo = from_slice(&body_bytes).unwrap();

    assert_eq!(body.title, updated_todo.title);
    assert_eq!(body.description, updated_todo.description);
    assert_eq!(body.is_done, updated_todo.is_done);

    let non_existent_id = Uuid::new_v4();
    let response = update_todo(
        State(state.clone()),
        Path(non_existent_id),
        Json(updated_todo),
    ).await;

    assert!(response.is_err());
}

#[tokio::test]
async fn test_delete_todo() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    let pool = PgPool::connect(&url).await.unwrap();

    let state = AppState { pool: pool.clone() };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let inserted_todo: Todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todo (title, description, is_done, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING id, title, description, is_done, created_at, updated_at
        "#
    )
        .bind("Todo to be deleted".to_string())
        .bind("This todo will be deleted".to_string())
        .bind(true)
        .fetch_one(&pool)
        .await
        .unwrap();

    let response = delete_todo(State(state.clone()), Path(inserted_todo.id))
        .await
        .unwrap()
        .into_response();

    let status = response.status();

    assert_eq!(status, StatusCode::NO_CONTENT);

    let deleted_todo = sqlx::query!(
        "SELECT id FROM todo WHERE id = $1",
        inserted_todo.id
    )
        .fetch_optional(&pool)
        .await
        .unwrap();

    assert!(deleted_todo.is_none());

    let non_existent_id = Uuid::new_v4();
    let response = delete_todo(State(state.clone()), Path(non_existent_id))
        .await;

    assert!(response.is_err());
}