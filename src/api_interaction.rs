use reqwest::{header::{self,HeaderMap}, Error, StatusCode};
use serde_json::json;
use base64::{Engine as _, engine::{general_purpose}};
use std::collections::HashMap;
use octocrab::{params, Octocrab, Page};  
use octocrab::models::issues::Issue;

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

    pub async fn post_issue(&self, title: &str, body: &str) -> Result<StatusCode, Error>{
        let payload = json!({
            "title": title,
            "body": body,
        });
        
        let client = reqwest::Client::new();

        let url = format!("{}/repos/{}/issues", std::env::var("INPUT_API_URL").unwrap(), std::env::var("INPUT_REPO").unwrap());

        let resp = client.post(url)
            .headers(self.headers.clone())
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        let resp_body = resp.text().await?;
        println!("Response body: {}", resp_body);

        Ok(status)
    }

    pub async fn get_issues() -> Result<Page<Issue>,octocrab::Error >{
        let repo = std::env::var("INPUT_REPO").unwrap();
        let owner: Vec<&str> = repo.split("/").collect();
        let octocrab = octocrab::instance();
        let issues = octocrab.issues(owner[0], owner[1]).list().send().await?;

        Ok(issues)
    }    

    // Git trees api??
    // Returns a hashmap with the name of the file as key and the content as value
    // the value will be a vector with each element representing a line of the file
    pub async fn get_files(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        let client = reqwest::Client::new();

        //default should be an empty string
        //TODO: What if the main branch has some other name?
        //Recurisve equal to something else?
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
        
        //Create the hashmap
        let mut files = std::collections::HashMap::new();

        for item in tree.as_array().unwrap() {
            //Checks if the item is a file
            if item.get("type").unwrap().as_str().unwrap() != "blob" {
                continue;
            }
            let url = item.get("url").unwrap().as_str().unwrap();
            let name = item.get("path").unwrap().as_str().unwrap();

            //get the content of the file
            let filereq = client.get(url)
                .headers(self.headers.clone())
                .send()
                .await?;
            
            let file_resp = filereq.text().await?;

            let file_json : serde_json::Value = match serde_json::from_str(&file_resp){
                Ok(val) => val,
                Err(e) => continue
            };

            //check if content exists
            //Might not be neeeded
            if !file_json.get("content").is_some() {
                continue;
            }

            
            let content = file_json.get("content").unwrap().as_str().unwrap();
            //The content is encoded in base64 but the newlines are not encoded. So we handle that by splitting the lines and decoding them one by one
            //Get the lines of the file
            let temp_lines: Vec<&str> = content.split('\n').collect();

            let mut lines = Vec::new();
            //JAG HAR INGEN ANING VARFÖR DET HÄR FUNKAR
            //FÖR NÅGONG ANLEDNING FIXAR DET NEWLINE PROBLEMET OCH DECODAR RÄTT
            for chunk in temp_lines.chunks(temp_lines.len()) {
                let concatenated = chunk.join("");
                lines.push(concatenated);
        }

            //Decode the lines
            let mut decoded_lines = Vec::new();
            //Decode each line
            for line in lines {
                //base64:decode might be deprecated
                let decoded_line = general_purpose::STANDARD.decode(line).unwrap();
                //convert from utf8 to string and put it in the vector
                decoded_lines.push(String::from_utf8(decoded_line).unwrap());
            }

            //put the files in the hashmap
            files.insert(String::from(name), decoded_lines);

        }
        
        //Return the hashmap
        Ok(files)
    }
}
