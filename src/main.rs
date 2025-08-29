use hyper::{Body, Request, Response, Server};
use dotenvy;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use sqlx::PgPool;
use std::sync::Arc;
mod models;

mod handlers;
use handlers::{
 get_user_tasks,
 get_users, 

 create_task, create_user
};

async fn handle(_req: Request<Body>, pool: Arc<PgPool>) -> Result<Response<Body>, Infallible> {
    match (_req.method().as_str(), _req.uri().path()) {
        ("POST", "/users") => create_user(_req, pool).await,
        ("GET", "/users") => get_users(_req, pool).await,
        ("POST", "/tasks") => create_task(_req, pool).await,
        ("GET", "/tasks") => get_user_tasks(_req, pool).await,
        _ => Ok(Response::new(Body::from("Not Found"))),
    }
}








#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
   let database_url = "postgresql://postgres:pass@localhost:5432/rust_db";

   // i know i have hardcoded this url it won`t work when u are seeing this 
    print!("Database url {:?}",database_url);
    let pool = Arc::new(PgPool::connect(&database_url).await?);

    sqlx::query(r#"
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);
"#)
.execute(&*pool)
.await
.unwrap();

sqlx::query(r#"
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT false
);
"#)
.execute(&*pool)
.await
.unwrap();


    let make_svc = make_service_fn({
        let pool = pool.clone();
        move |_conn| {
            let pool = pool.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let pool = pool.clone();
                    handle(req, pool)
                }))
            }
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
