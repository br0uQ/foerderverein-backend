use axum::{
    extract, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod types;
use types::mailerState;
use types::GetData;
use types::Person;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .route("/data", post(create_user))
        .route("/send", get(mail).post(send_mail))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_people() -> impl IntoResponse {
    println!("called get_people()");
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
            id: 1,
        },
        Person {
            name: String::from("Person B"),
            age: 6,
            favourite_food: Some(String::from("Broccoli")),
            id: 0,
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
            id: 2,
        },
    ];

    (StatusCode::OK, Json(people))
}

#[derive(serde::Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
struct SentResp {
    status: String,
}

async fn create_user(extract::Json(payload): extract::Json<CreateUser>) -> impl IntoResponse {
    println!("{:?}", payload.username);
    let stat = SentResp {
        status: String::from("Email sent"),
    };

    (StatusCode::CREATED, Json(stat))
}

async fn send_mail(extract::Json(payload): extract::Json<mailerState>) -> impl IntoResponse {
    println!("{:?}", payload);
    let user = User {
        id: 1337,
        username: payload.name,
    };

    StatusCode::OK
}

async fn mail() -> impl IntoResponse {
    println!("called get_people()");
    let user = User {
        id: 1337,
        username: String::from("myname"),
    };

    (StatusCode::OK, Json(user))
}

async fn get_data(extract::Json(payload): extract::Json<GetData>) {
    println!("called get_data");
    println!("{:?}", payload);
}
