use axum::Router;

mod day1;
mod day4;
mod day5;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(day1::day1())
        .merge(day5::day5())
        .merge(day4::day2());

    Ok(router.into())
}
