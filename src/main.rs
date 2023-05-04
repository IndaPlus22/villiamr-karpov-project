use std::env;
use reqwest;
fn main() {
    let vars = vec!["INPUT_REPO", "INPUT_LATEST_PUSH", "INPUT_COMMITS", "INPUT_DIFF_URL", "INPUT_API_URL"];

    for v in vars {
        match env::var(v) {
           Ok(val) => println!("{}: {}", v, val),
           Err(e)  => println!("Variable {} not set: {}", v ,e)
        }
    }
    let token = match env::var("GITHUB_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
//Might be able to make asynchonous
// fn create_issue() {
//     let client = reqwest::Client::new();
//     let res = client.post()
//     .header(ACCEPT, "application/vnd.github.v3+json")
//     .header(AUTHORIZATION, "token ${{ secrets.GITHUB_TOKEN }}")

// }