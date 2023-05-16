use reqwest::{header::{self,HeaderMap}, Error, StatusCode};
use serde_json::json;
use base64::{Engine as _, engine::{general_purpose}};
use std::collections::HashMap;
use octocrab::{params, Octocrab, Page};  
use octocrab::models::issues::Issue;
use std::fs;

pub struct GithubApiClient {
    client: Octocrab,
    owner: String,
    repo: String
}

impl GithubApiClient {
    pub fn new() -> Result<GithubApiClient, octocrab::Error> {
        let repo = std::env::var("INPUT_REPO").unwrap();
        let owner_and_repo: Vec<&str> = repo.split("/").collect();
        return Ok(GithubApiClient {
            client: octocrab::OctocrabBuilder::default()
                .personal_token(std::env::var("INPUT_TOKEN").unwrap())
                .base_uri(std::env::var("INPUT_API_URL").unwrap())?
                .build()?,
            owner: owner_and_repo[0].to_string(),
            repo: owner_and_repo[1].to_string()
        });
    }

    //TODO: Issues should be constructed inside the parsing function
    //Alternatively function could take arguments for lables, assignies and so on as options
    pub async fn post_issue(&self,title: &str, body: &str) -> Result<Issue, octocrab::Error>{
        let issue = self.client
            .issues(self.owner.clone(), self.repo.clone())
            .create(title)
            .body(body)
            .send()
            .await?;

        Ok(issue)
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
