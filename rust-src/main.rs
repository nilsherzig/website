use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            post(create_user).layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(
                        "http://localhost:5173"
                            .parse::<axum::http::HeaderValue>()
                            .unwrap(),
                    )
                    .allow_headers([CONTENT_TYPE])
                    .allow_methods([axum::http::Method::POST]),
            ),
        )
        .route(
            "/get",
            get(get_user).layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(
                        "http://localhost:5173"
                            .parse::<axum::http::HeaderValue>()
                            .unwrap(),
                    )
                    .allow_headers([CONTENT_TYPE])
                    .allow_methods([axum::http::Method::GET]),
            ),
        );
    async fn get_user() -> (StatusCode, Json<User>) {
        let user = User {
            id: 1338,
            username: "random-username".to_string(),
        };

        (StatusCode::CREATED, Json(user))
    }

    async fn create_user(
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Json(payload): Json<CreateUser>,
    ) -> (StatusCode, Json<User>) {
        // insert your application logic here
        let user = User {
            id: 1337,
            username: payload.username,
        };

        // this will be converted into a JSON response
        // with a status code of `201 Created`
        (StatusCode::CREATED, Json(user))
    }

    // the input to our `create_user` handler
    #[derive(Deserialize)]
    struct CreateUser {
        username: String,
    }

    // the output to our `create_user` handler
    #[derive(Serialize)]
    struct User {
        id: u64,
        username: String,
    }

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
