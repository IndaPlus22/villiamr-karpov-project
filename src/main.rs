use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let title = "Found a bug";
    let body = "I'm having a problem with this.";
    let token = "your-github-token-here";

    let payload = json!({ "title": title, "body": body });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/repos/IndaPlus22/villiamr-karpov-project/issues")
        .header("Authorization", format!("token {}", token))
        .json(&payload)
        .send()
        .await?;

    println!("Response status: {}", res.status());

    Ok(())
}
