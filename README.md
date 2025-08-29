# Rust Hyper SQLx Backend

A simple backend API built with **Rust**, using **Hyper** for HTTP handling and **SQLx** for Postgres database interactions. This project demonstrates how to create users and tasks with database persistence, JSON handling using **Serde**, and async request processing.

---

## Features

- Minimal HTTP server using **Hyper** (no web framework).  
- PostgreSQL database connection via **SQLx**.  
- Async request handling with **Tokio**.  
- CRUD operations for:
  - **Users**
  - **Tasks**
- JSON serialization/deserialization using **Serde**.  
- Dynamic table creation if missing (`users` and `tasks`).  

---

## Project Structure

```
├── Cargo.toml
├── src
│ ├── main.rs # Entry point, server setup
│ ├── models.rs # Data models and structs
│ └── handlers.rs # Request handlers for each endpoint
├── .env # Environment variables (DATABASE_URL)
└── README.md
```

---

## Dependencies

```toml
[dependencies]
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenvy = "0.15"
```


Database Schema

The project creates two tables automatically if they do not exist:

```
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT false
);

```


Running the Server

1.Ensure Postgres is running and your database exists (rust_db).

2.Run the server:
```
cargo run
```

3.Server listens on 127.0.0.1:3000



