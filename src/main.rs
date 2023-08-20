use axum::{
    extract, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod mail;
mod types;
use types::MailerState;
use types::Person;
use types::User;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .route("/send", post(send_mail))
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

async fn send_mail(extract::Json(payload): extract::Json<MailerState>) -> impl IntoResponse {
    println!("{:?}", payload);
    let user = User {
        id: 1337,
        username: payload.name.clone(),
    };

    let mut subject = String::from("Nachricht von: '");
    subject.push_str(payload.name.as_str());
    subject.push_str("' <");
    subject.push_str(payload.email.as_str());
    subject.push_str(">");

    let result = mail::send_mail(subject, payload.message);
    match result {
        Ok(_) => println!("Successfully sent email"),
        Err(e) => println!("{}", e),
    }

    (StatusCode::OK, Json(user))
}
