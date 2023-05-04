use std::env;

fn main() {
    let vars = vec!["INPUT_REPO", "INPUT_LATEST_PUSH", "INPUT_COMMITS", "INPUT_DIFF_URL", "INPUT_API_URL"];

    for v in vars {
        match env::var(v) {
           Ok(val) => println!("{}: {}", v, val),
           Err(e)  => println!("Variable {} not set: {}", v ,e)
        }
    }
}
