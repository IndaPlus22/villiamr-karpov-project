use octocrab::{OctocrabBuilder, Octocrab, issues::{IssueHandler}, params, current};
use octocrab::models::issues::Issue;
use octocrab::Page;
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

    //TODO: Issues should be constructed inside the parsing function
    //Alternatively function could take arguments for lables, assignies and so on as options
    pub async fn post_issue(&self,title: &str, body: &str) -> Result<Issue, octocrab::Error>{
        let issue = self.client
            .issues(self.owner.clone(), self.repo.clone())
            .create(title)
            .body(body)
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
                    //TODO: Atleast check if bodys are equal
                    *map.get_mut(&issue.title).unwrap() = self.lable_as_duplicate(&map[&issue.title]).await?;
                    self.lable_as_duplicate(issue).await?;
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
    
    //TODO: Investigate if duplicate lables are a thing?
    async fn lable_as_duplicate(&self, issue: &Issue) -> Result<Issue,octocrab::Error> {
        println!("Issue number: {}, marked as duplicate", issue.number);
        let new_issue = self.client.issues(self.owner.clone(), self.repo.clone())
            .update(issue.number)
            .labels(&[String::from("duplicate")])
            .send()
            .await?;

        Ok(new_issue)
    } 
}
