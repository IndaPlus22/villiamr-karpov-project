use reqwest::{header::{self,HeaderMap}, Error, StatusCode};
use serde_json::json;
use base64::{Engine as _, engine::{general_purpose}};
use std::collections::HashMap;
use octocrab::{params, Octocrab, Page};  
use octocrab::models::issues::Issue;
use std::fs;

pub struct GithubApiClient {
    headers: HeaderMap
}

impl GithubApiClient {
    pub fn new() -> GithubApiClient {
        let mut header = HeaderMap::new();
        header.insert(header::USER_AGENT, header::HeaderValue::from_static("TODO ACTION"));
        header.insert(header::AUTHORIZATION,format!("token {}",std::env::var("INPUT_TOKEN").unwrap()).parse().unwrap());
        header.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));


        return GithubApiClient{
            headers: header, 
        };
    }

    pub async fn post_issue(title: &str, body: &str) -> Result<Issue, octocrab::Error>{
        let repo = std::env::var("INPUT_REPO").unwrap();
        let owner: Vec<&str> = repo.split("/").collect();
        let test = octocrab::instance().issues(owner[0], owner[1]).create(title).body(body).send().await?;
//        let payload = json!({
//            "title": title,
//            "body": body,
//        });
//        
//        let client = reqwest::Client::new();
//
//        let url = format!("{}/repos/{}/issues", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap());
//
//        let resp = client.post(url)
//            .headers(self.headers.clone())
//            .json(&payload)
//            .send()
//            .await?;
//
//        let status = resp.status();
//        let resp_body = resp.text().await?;
//        println!("Response body: {}", resp_body);

        Ok(test)
    }

    pub async fn get_issues() -> Result<Page<Issue>,octocrab::Error >{
        let repo = std::env::var("INPUT_REPO").unwrap();
        let owner: Vec<&str> = repo.split("/").collect();
        let issues = octocrab::instance()
            .issues(owner[0], owner[1])
            .list()
            .send()
            .await?;

        Ok(issues)
    }    
}
