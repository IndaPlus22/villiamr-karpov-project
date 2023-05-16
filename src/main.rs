mod api_interaction;

use std::{fs, path::PathBuf};

/* Supported languages include c, c++, java, rust, javascript (Should be extendable to all c-style
 * comments without any changes more than adding them to list of extensions. We have decided
 * against to make sure our project does not destroy someones repo in case of unforseen side
 * effects)
 *   
 *   Issue creation format (_ indicates abritrary whitespace):
 *
 *   // _ TODO _ : _ Title _ \n
 *   //_ Body _ \n 
 *   //  ....
 *
 */

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

fn walk_dirs(base_dir: PathBuf){
    println!("Searching dir: {}", base_dir.display());
    let paths = fs::read_dir(&base_dir).unwrap();

    //TODO: Extend list of valid file extensions
    let valid_extensions = vec!["c","h","cpp","hpp","rs"]; 
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            let mut req_dir = base_dir.clone();
            req_dir.push(path.as_ref().unwrap().path());
            walk_dirs(req_dir);
            continue;
        }
        // If it's not a dir
        if let Some(extension) = get_fileextension(path.as_ref().unwrap().file_name().to_str().unwrap()){
            if valid_extensions.contains(&extension) {
                println!("Parsing file {}", path.unwrap().path().display());
            } 
        }
    }
} 

#[tokio::main]
async fn main() -> Result<(), octocrab::Error>{
    walk_dirs(PathBuf::from("/github/workspace/"));    


    //TODO: Den här retunerar okej 
    //om detta issue finns
    //Fungerar skiten
    Ok(())
}
