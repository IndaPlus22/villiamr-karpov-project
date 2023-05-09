use reqwest::{header::{self,HeaderMap}, Error, StatusCode};
use serde_json::json;

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

    pub async fn post_issue(self, title: String, body: String) -> Result<StatusCode, Error>{
        let payload = json!({
            "title": title,
            "body": body,
        });
        
        let client = reqwest::Client::new();

        let url = format!("{}/repos/{}/issues", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap());

        let resp = client.post(url)
            .headers(self.headers)
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        let resp_body = resp.text().await?;
        println!("Response body: {}", resp_body);

        Ok(status)
    }

    // Git trees api??
    pub async fn get_files(self) -> Result<StatusCode, Error> {
        let client = reqwest::Client::new();

        let url = format!("{}/repos/{}/contents/", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap());

        let resp = client.get(url)
            .headers(self.headers)
            .send()
            .await?;
        
        let status = resp.status();
        let resp_body = resp.text().await?;

        //Get the url for each file/directory
        let json: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        
        for item in json.as_array().unwrap() {
            let url = item.get("url").as_str().unwrap();
            println!("Url: {}", url)
        }
        

        //Check for errors here

        //print response body
        //println!("Response body: {}", resp_body);

        //Return status
        Ok(status)
    }
}
