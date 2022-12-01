use std::{env, error::Error, sync::Arc};

use reqwest::{blocking::Client, cookie::Jar, Url};

fn get_aoc_client() -> Result<Client, Box<dyn Error>> {
    let cookie = format!("session={}", env::var("AOC_SESSION")?);

    let url = "https://adventofcode.com".parse::<Url>()?;

    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder().cookie_provider(jar).build()?;

    Ok(client)
}

/// download puzzle input
/// TODO: setup input caching
pub fn get_input(year: i32, day: i32) -> Result<String, Box<dyn Error>> {
    println!("Downloading input for {year}-{day}");
    let client = get_aoc_client()?;

    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .send()?;

    let result = res.text()?;

    Ok(result)
}
