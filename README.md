# Basic Web App in Rust 🚀

This is a simple, beginner-friendly web application built with [Rust](https://www.rust-lang.org/) using the [Axum](https://docs.rs/axum) web framework.

## 📦 Features

- HTTP server with Axum
- REST-style `GET` and `POST` routes
- JSON input/output with Serde
- Modular code structure (`routes/`, `models/`)
- Clean and extendable project layout

## 🛠 Tech Stack

- Rust
- Axum (Web Framework)
- Tokio (Async Runtime)
- Serde (for JSON serialization)

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/tahseenjamal/basic-web-app-in-rust.git
cd basic-web-app-in-rust
```

### 2. Run the app

```bash
cargo run
```

Server will start at: [http://127.0.0.1:3000](http://127.0.0.1:3000)

## 📁 Project Structure

```
src/
├── main.rs             # Entry point
├── routes/             # All route bindings
│   └── routes.rs       # Defines application routes
├── models/             # Data models and request handlers
│   ├── blog.rs
│   └── user.rs
```

## 📬 API Endpoints

| Method | Route    | Description               |
|--------|----------|---------------------------|
| POST   | `/blog`  | Create a new blog post    |
| GET    | `/blog`  | Retrieve a sample blog    |
| POST   | `/user`  | Create a new user         |
| GET    | `/user`  | Retrieve a sample user    |

## 🔧 Example Payloads

### POST `/blog`

```json
{
  "user": {
    "username": "tahseen",
    "name": "Tahseen Jamal",
    "created": 3434
  },
  "tweet": "Hello World!",
  "timestamp": 3434
}
```

### POST `/user`

```json
{
  "username": "darkstar",
  "name": "Jamal",
  "created": 3434
}
```

