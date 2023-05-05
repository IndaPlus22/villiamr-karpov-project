use std::env;
use reqwest;

async fn main() {
    let vars = vec!["INPUT_REPO", "INPUT_LATEST_PUSH", "INPUT_COMMITS", "INPUT_DIFF_URL", "INPUT_API_URL"];

    for v in vars {
        match env::var(v) {
           Ok(val) => println!("{}: {}", v, val),
           Err(e)  => println!("Variable {} not set: {}", v ,e)
        }
    }
    let token = match env::var("INPUT_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    create_issue(&token).await?;
    Ok(())
}

//Might be able to make asynchonous
async fn create_issue(token: &str) -> Result<(), reqwest::Error>{
    let body = r#"{"title":"Found a bug","body":"I'm having a problem with this."}"#;
    let client = reqwest::Client::new();
    let res = client
    .post("https://github.com/IndaPlus22/villiamr-karpov-project/issues")
    .header("ACCEPT", "application/vnd.github.v3+json")
    .header("AUTHORIZATION", format!("token {}", token))
    .header("X-GITHUB-API-VERSION", "2022-11-28")
    .body(body)
    .send()
    .await?;
    //println!("Response status: {}", res.status());
    let body1 = res.text().await?;
    println!("Response body: {}", body);
    Ok(())
}