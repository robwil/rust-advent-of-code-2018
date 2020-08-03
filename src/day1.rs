use reqwest;
use tokio::runtime::Runtime;

async fn get_input() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://adventofcode.com/2018/day/1/input")
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn day1() {
    let input = Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(get_input())
        .expect("Failed to fetch problem input");
    println!("{}", input);
}
