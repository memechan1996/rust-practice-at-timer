pub mod problem;

use crate::problem_mgr::problem::Problem;
use std::collections::HashMap;
use rand::seq::SliceRandom;

pub struct ProblemMgr{
    problems: HashMap<String, Problem>,
    client: reqwest::Client,
}

impl ProblemMgr{
    pub async fn new() -> Self{
        let url = "https://kenkoooo.com/atcoder/resources/problem-models.json";
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/137.0.0.0 Safari/537.36"
            )
            .gzip(true)
            .build()
            .unwrap();

        let text = client
            .get(url)
            .header("Accept-Encoding", "gzip, deflate, br")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        Self { 
            problems: serde_json::from_str(&text).unwrap(),
            client,
        }
    }

    pub fn get_problem_id(&self, id:&str) -> Option<&Problem>{
        self.problems.get(id)
    }

    pub fn get_problem_random_min_max(&self, min_diff: f32, max_diff: f32) -> Option<String> {
        let candidates: Vec<&String> = self.problems
            .iter()
            .filter_map(|(id, model)| {
                let diff = model.difficulty? as f32;
                if diff >= min_diff && diff <= max_diff {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        candidates.choose(&mut rand::thread_rng())
            .map(|id| id.to_string())
    }

    pub async fn get_problem_url(&self, id:&str) -> Option<String>{
        let contest = id.split('_').next()?;
        let url = format!("https://atcoder.jp/contests/{}/tasks/{}", contest, id);

        let res = self.client.get(&url).send().await.ok()?;

        if res.status().is_success() {
            Some(url)
        } else {
            None
        }
    }
}
