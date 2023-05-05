use reqwest::*;
use std::error::Error;

fn main(){

    let token = std::env::var("INPUT_TOKEN");
    let title = "Found a bug";
    let body = "I'm having a problem with this.";
    let payload = serde_json::json!({ "title": title, "body": body });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.github.com/repos/IndaPlus22/villiamr-karpov-project/issues")
        .header("Authorization", format!("token {:?}", token))
        .json(&payload)
        .send()
        .unwrap();

    println!("Response status: {}", res.status());
    let body = res.text();
    println!("Response body:\n{:?}", body);
}
