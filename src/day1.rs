use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn power_of_three(Path(path): Path<String>) -> impl IntoResponse {
    let numbers = path.split('/').take(20).filter_map(|x| x.parse::<i32>().ok());
    let x = numbers.reduce(|acc, next| acc ^ next).unwrap();
    let y = x.pow(3);

    (StatusCode::OK, y.to_string())
}

pub fn day1() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(server_error))
        .route("/1/*key", get(power_of_three))
}
