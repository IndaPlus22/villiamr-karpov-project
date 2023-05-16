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
    use std::fs;

    let paths = fs::read_dir("/github/workspace/").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }

    //TODO: Den här retunerar okej 
    //om detta issue finns
    //Fungerar skiten
    Ok(())
}
