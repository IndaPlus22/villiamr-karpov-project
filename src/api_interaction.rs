use octocrab::{Octocrab, params };
use octocrab::models::issues::Issue;
use std::collections::HashMap;

pub struct GithubApiClient {
    client: Octocrab,
    owner: String,
    repo: String
}

impl GithubApiClient {
    pub fn new() -> Result<GithubApiClient, octocrab::Error> {
        let repo = std::env::var("INPUT_REPO").unwrap();
        let owner_and_repo: Vec<&str> = repo.split("/").collect();
        return Ok(GithubApiClient {
            client: octocrab::OctocrabBuilder::default()
                .personal_token(std::env::var("INPUT_TOKEN").unwrap())
                .base_uri(std::env::var("INPUT_API_URL").unwrap())?
                .build()?,
            owner: owner_and_repo[0].to_string(),
            repo: owner_and_repo[1].to_string()
        });
    }

    //TODO: Issues could be constructed inside the parsing function
    //Alternatively function could take arguments for lables, assignies and so on as options
    //Currently bare issues are created: title, body and a lable
    pub async fn post_issue(&self,title: &str, body: &str) -> Result<Issue, octocrab::Error>{
        let issue = self.client
            .issues(self.owner.clone(), self.repo.clone())
            .create(title)
            .body(body)
            .labels(vec![String::from("TODO")])
            .send()
            .await?;

        Ok(issue)
    }

    //Gets all open issues (hashed by title) and marks duplicates, closed issues are ignored.
    pub async fn process_issues(&self) -> Result<HashMap<String,Issue>,octocrab::Error >{
        let mut page = self.client.issues(self.owner.clone(), self.repo.clone())
            .list()
            .state(params::State::Open)
            .send()
            .await?;

        let mut map: HashMap<String,Issue> = HashMap::new();
        loop {
            for issue in &page {
                if !map.contains_key(&issue.title){
                    map.insert(issue.title.clone(), issue.clone());
                } 
                else {
                    // If issue exists and has equal body to other we mark both as dupicate
                    if issue.body == map[&issue.title].body {
                        *map.get_mut(&issue.title).unwrap() = self.lable_issue(&map[&issue.title], String::from("duplicate")).await?;
                        self.lable_issue(issue, String::from("duplicate")).await?;
                    }
                    // If only title are equal we flag that too
                    else {
                        *map.get_mut(&issue.title).unwrap() = self.lable_issue(&map[&issue.title], String::from("duplicate title")).await?;
                        self.lable_issue(issue, String::from("duplicate title")).await?;
                    }
                }
            }

            page = match self.client
                .get_page::<Issue>(&page.next)
                .await? 
            {
                Some(next_page) => next_page,
                None => break,
                
            } 
        }
        

        Ok(map)
    } 
    
    //TODO: Investigate if duplicate lables are a thing? could be an edge case with >2 duolicated issues
    async fn lable_issue(&self, issue: &Issue,lable: String) -> Result<Issue,octocrab::Error> {
        println!("Issue number: {}, marked as duplicate", issue.number);
        let new_issue = self.client.issues(self.owner.clone(), self.repo.clone())
            .update(issue.number)
            .labels(&[lable])
            .send()
            .await?;

        Ok(new_issue)
    } 
}
