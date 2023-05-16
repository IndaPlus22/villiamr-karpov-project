mod api_interaction;

use std::{fs::{self, File}, path::PathBuf, io::Read, str::Chars};

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

fn parse_todo(iterator: &mut Chars, index: &mut usize) -> Option<(String, String)> {
    let mut title = String::new();
    let mut body = String::new();

    while let Some(c) = iterator.next() {
        *index += c.len_utf8();
        match c {
            ' ' => continue, 
            _ => title.push(c)
        }
        if title.len() > 5 {return None;}
        if title.to_lowercase() == "todo:" {
            title.clear();
            break;
        }
    }
    // At this point we have a todo that will become an issue unless it is a duplicate
    while let Some(c) = iterator.next() {
        *index += c.len_utf8();
        if c == '\n' {break;}
        title.push(c);
    }
    title = String::from(title.trim());

    let mut slash_count = 0;
    while let Some(c) = iterator.next() {
        *index += c.len_utf8();
        match c {
            '/' => slash_count += 1,
            '\n' => if slash_count < 2 {break;} // No comment on this line
                    else {slash_count = 0; body.push(' ')}
            _ => if slash_count >= 2 {
                body.push(c);
            } 
        }
    }

    Some((title,body))
}

fn parse_file(path: &str) -> Vec<(String, String)>{
    let mut todos = Vec::new();
    let mut content = String::new();
    let file = File::open(path).expect("Could not open file").read_to_string(&mut content).unwrap();

    //TODO: Current edge cases are string litterals like "//TODO: ..." may find more
    let mut string_flag = false;
    let mut buffer = String::new();
    let mut curr_index = 0;
    let mut iterator = content.chars();
    while let Some(c) = iterator.next() {
        curr_index += c.len_utf8();
        match c {
            '"' => string_flag = !string_flag,
            '/' => if !string_flag {buffer.push(c);},
            _ => if buffer.len() > 0 {buffer.clear();},
        }         

        if buffer.len() == 2 && buffer == "//" { //Need to check that buffer is a comment since len
                                                 //can be 2 whith just one characer
            if let Some(todelio) = parse_todo(&mut iterator, &mut curr_index) {
                todos.push(todelio);
            } 
                       
        }
    }        
    
    return todos;
}



fn walk_dirs(base_dir: PathBuf, client: &api_interaction::GithubApiClient){ 
    println!("Searching dir: {}", base_dir.display());
    let paths = fs::read_dir(&base_dir).unwrap();

    //TODO: Extend list of valid file extensions
    let valid_extensions = vec!["c","h","cpp","hpp","rs"]; 
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            let mut req_dir = base_dir.clone();
            req_dir.push(path.as_ref().unwrap().path());
            walk_dirs(req_dir, client);
            continue;
        }
        // If it's not a dir
        if let Some(extension) = get_fileextension(path.as_ref().unwrap().file_name().to_str().unwrap()){
            if valid_extensions.contains(&extension) {
                println!("Parsing file {}", path.as_ref().unwrap().path().display());
                println!("{:#?}", parse_file(path.as_ref().unwrap().file_name().to_str().unwrap()));
            } 
        }
    }
} 

#[tokio::main]
async fn main() -> Result<(), octocrab::Error>{
    let client = api_interaction::GithubApiClient::new().await?;
    walk_dirs(PathBuf::from("/github/workspace/"), &client);    


    //TODO: Den här retunerar okej 
    //om detta issue finns
    //Fungerar skiten
    Ok(())
}
