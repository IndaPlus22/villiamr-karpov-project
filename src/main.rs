use std::{env, string};

struct ApiInteraction {
    token: String,
    url: String
}

impl ApiInteraction {
    fn new(token_: String, baseurl: String, repo: String) -> ApiInteraction {
        return ApiInteraction{token:token_, url:format!("{}{}/issues",baseurl, repo)};
    }
}

fn main() {    
    let api = ApiInteraction::new(
        std::env::var("INPUT_TOKEN").unwrap(),
        std::env::var("INPUT_API_URL").unwrap(), 
        std::env::var("INPUT_REPO").unwrap());
    
    println!("{}",api.url);
}
