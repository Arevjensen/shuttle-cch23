use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn serve_asset(Path(path): Path<String>) -> impl IntoResponse {
    let numbers = path.split('/').filter_map(|x| x.parse::<u32>().ok());
    let x = numbers.fold(0, |acc, next| acc ^ next);
    let y = x.pow(3);

    (StatusCode::OK, y.to_string())
}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(server_error))
        .route("/1/*key", get(serve_asset));

    Ok(router.into())
}
