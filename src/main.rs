use std::env;

fn main() {
    let token = std::env::var("INPUT_TOKEN");
    println!("{:?}", token);
}
