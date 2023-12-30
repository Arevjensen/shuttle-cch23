use axum::Router;

mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(day1::day1())
        .merge(day4::day2())
        .merge(day5::day5())
        .merge(day8::day8())
        .merge(day6::day6())
        .merge(day7::day7());

    Ok(router.into())
}
