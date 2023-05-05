use std::{env, string};
use serde_json::{json};
use reqwest::{self, header::{HeaderMap, self}, Error};

struct ApiInteraction {
    token: String,
    url: String
}

impl ApiInteraction {
    fn new() -> ApiInteraction {
        return ApiInteraction{
            token:std::env::var("INPUT_TOKEN").unwrap(), 
            url:format!("{}/repos/{}/issues", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap())
        };
    }

    async fn post_issue(self, title: String, body: String) -> Result<(), Error>{
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("TODO ACTION"));
        headers.insert(header::AUTHORIZATION,format!("token {}",self.token).parse().unwrap());
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

        let playload = json!({
            "title": title,
            "body": body,
        });
        
        println!("{:?}",headers);

        let client = reqwest::Client::new();

        let resp = client.post(self.url)
            .headers(headers)
            .json(&playload)
            .send()
            .await?;

        println!("Status: {}", resp.status());

        let resp_body = resp.text().await?;
        println!("Response body: {}", resp_body);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error>{    
    let api = ApiInteraction::new();
    api.post_issue("First issue".to_string(), "Generated by action if successfull".to_string()).await?;
    Ok(())
}
