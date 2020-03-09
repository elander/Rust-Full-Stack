#[macro_use]
extern crate fstrings;

use reqwest;
use serde_json::Value;

// use console::Style;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://www.reddit.com/r/rust/new/.json?count=50
    let target = "rust";
    let sort = "new";
    let limit = "50";

    let payload = f!("https://www.reddit.com/r/{target}/{sort}/.json?count={limit}");
    // println!("{}", &payload);
    let body_text = reqwest::get(&payload).await.unwrap().text().await.unwrap();
    // println!("Body\n {:#?}", &body_text);

    // Only extract what you want.
    // https://www.google.com/search?&q=extract+only+json+parts+I+want+serde

    let json_value: Value = serde_json::from_str(&body_text).unwrap();
    // println!("JSON\n {:#?}", &json_value);

    let list_of_posts = json_value["data"]["children"]
    println!("List of Posts\n {:#?}", &list_of_posts);

    Ok(())
}


