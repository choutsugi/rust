## 一、serde
依赖引入：
```toml
[dependencies]
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"
```
序列化与反序列化：
```rust
use serde::{Deserialize, Serialize};
use serde_json as sj;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    // 反序列化
    let s = r#"
        {"name": "alex", "age": 30}
    "#;
    let v: User = sj::from_str(s).unwrap();
    println!("{:?}", v);

    // 序列化
    let u = User {
        name: "Aaron".into(),
        age: 40,
    };
    let s = sj::to_string(&u).unwrap();
    println!("{}", s);
}
```
## 二、Axum请求
依赖：
```toml
[package]
name = "axum"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.2"

serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"

tokio = {version = "1.0", features = ["full"]}

tracing = "0.1"
tracing-subscriber = "0.2"
```
示例：
```rust
use axum::{
    extract::{Form, Path},
    handler::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, str::FromStr};
use tokio::runtime::Builder;

fn main() {
    // 构建多线程运行时
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let _my_root = root;
    let _my_create_user_json = create_user_json;

    // 异步阻塞
    rt.block_on(async {
        let app = Router::new()
            .route("/", get(root))
            .route("/json", post(create_user_json))
            .route("/url/:name/:age", post(create_user_get))
            .route("/form", post(create_user_form));

        let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

// GET
async fn root() -> &'static str {
    "Hello, World!"
}

// POST with Json
async fn create_user_json(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1000,
        name: payload.name,
        age: payload.age,
    };

    (StatusCode::CREATED, Json(user))
}

// POST with Path
async fn create_user_get(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").unwrap().clone();
    let age = params.get("age").unwrap().parse().unwrap();

    let user = User {
        id: 2000,
        name,
        age,
    };

    (StatusCode::CREATED, Json(user))
}

// POST with Form
async fn create_user_form(Form(input): Form<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 3000,
        name: input.name,
        age: input.age,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    age: i32,
}
```
测试（REST Client插件）：test.http
```http
GET http://127.0.0.1:3000/ HTTP/1.1

###

POST http://127.0.0.1:3000/json HTTP/1.1
Content-Type: application/json

{
    "name": "Alex",
    "age": 10
}

###

POST http://127.0.0.1:3000/url/Aaron/20 HTTP/1.1

###

POST http://127.0.0.1:3000/form HTTP/1.1
Content-Type: application/x-www-form-urlencoded

&name=Hello
&age=30
```
## 三、Axum中间件
