use hyper::{Body, Request, Response};
use sqlx::PgPool;
use std::sync::Arc;
use serde_json::{from_slice, to_string};
use crate::models::{NewUser, User, Task, NewTask};
use std::convert::Infallible;

pub async fn create_user( req : Request<Body>, pool :Arc<PgPool>)-> Result<Response<Body> , Infallible>{
    let body =hyper::body::to_bytes(req.into_body()).await.unwrap();
    let new_user: NewUser = from_slice(&body).unwrap();

     let row: (i32, String, String) = sqlx::query_as(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"
    )
    .bind(&new_user.name)
    .bind(&new_user.email)
    .fetch_one(&*pool)
    .await
    .unwrap();
 let user = User { id: row.0, name: row.1, email: row.2 };

    // Serialize Rust struct back to JSON
    let response_body = to_string(&user).unwrap();

    Ok(Response::new(Body::from(response_body)))
}


pub async fn get_users(_req: Request<Body>, pool: Arc<PgPool>) -> Result<Response<Body>, Infallible> {
    let rows: Vec<User> = sqlx::query_as("SELECT id, name, email FROM users")
        .fetch_all(&*pool)
        .await
        .unwrap();

    let response_body = to_string(&rows).unwrap();
    Ok(Response::new(Body::from(response_body)))
}



pub async fn create_task(req: Request<Body>, pool: Arc<PgPool>) -> Result<Response<Body>, Infallible> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let new_task: NewTask = from_slice(&body_bytes).unwrap();

    let row: (i32, String, i32, bool) = sqlx::query_as(
        "INSERT INTO tasks (title, user_id , completed ) VALUES ($1, $2, $3) RETURNING id, title, user_id, completed"
    )
    .bind(&new_task.title)
    .bind(new_task.user_id)
    .bind(new_task.completed)
    .fetch_one(&*pool)
    .await
    .unwrap();

    let task = Task { id: row.0, title: row.1, user_id: row.2, completed: row.3 };
    let response_body = to_string(&task).unwrap();

    Ok(Response::new(Body::from(response_body)))
}


pub async fn get_user_tasks(req: Request<Body>, pool: Arc<PgPool>) -> Result<Response<Body>, Infallible> {
    let query = req.uri().query().unwrap_or(""); // e.g., ?user_id=1
    let user_id: i32 = query.strip_prefix("user_id=").unwrap_or("0").parse().unwrap_or(0);

    let rows: Vec<Task> = sqlx::query_as("SELECT id, title, user_id , completed FROM tasks WHERE user_id=$1")
        .bind(user_id)
        .fetch_all(&*pool)
        .await
        .unwrap();

    let response_body = to_string(&rows).unwrap();
    Ok(Response::new(Body::from(response_body)))
}
