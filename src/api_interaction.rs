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

        //default should be an empty string
        //TODO: What if the main branch has some other name?
        let url = format!("{}/repos/{}/git/trees/main?recursive=1", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap());

        let resp = client.get(url)
            .headers(self.headers.clone())
            .send()
            .await?;
        
        let status = resp.status();
        let resp_body = resp.text().await?;

        //Get the url for each file
        let json: serde_json::Value = serde_json::from_str(&resp_body).unwrap();

        let tree = &json["tree"];
        
        for item in tree.as_array().unwrap() {
            //if item type is dir we need to run this function on that directory
            if item.get("type").unwrap().as_str().unwrap() != "blob" {
                continue;
            }
            let url = item.get("url").unwrap().as_str().unwrap();
            let name = item.get("path").unwrap().as_str().unwrap();
            println!("Name: {}", name);
            println!("Url: {}", url);

            //get the content and then print it
            let filereq = client.get(url)
                .headers(self.headers.clone())
                .send()
                .await?;
            
            let file_resp = filereq.text().await?;

            let file_json : serde_json::Value = serde_json::from_str(&file_resp).unwrap();

            let content = file_json.get("content").unwrap().as_str().unwrap();

            //convert from base64
            let decoded = base64::decode(content).unwrap();
            let decoded_str = String::from_utf8(decoded).unwrap();

            //print content
            println!("Content: {}", decoded_str);

        }
        

        //Check for errors here

        //print response body
        println!("Response body: {}", resp_body);

        //Return status
        Ok(status)
    }
}
