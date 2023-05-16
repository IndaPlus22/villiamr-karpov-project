mod api_interaction;
use reqwest::Error;

fn get_fileextension(s: &str) -> Option<&str> {
    let mut i = 0;
    for c in s.chars().rev() {
        if c == '.' {
            return Some(&s[(s.len() - i)..]); 
        }
        i+=1;
    }
    return None;
}

#[tokio::main]
async fn main() -> Result<(), octocrab::Error>{
    let issue = api_interaction::GithubApiClient::post_issue("Hello world", "Created with octocrab").await?;
    println!("{:#?}", issue);

    //TODO: Den här retunerar okej 
    //om detta issue finns
    //Fungerar skiten
    Ok(())
}
