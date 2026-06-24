pub mod problem;

use crate::problem_mgr::problem::{Problem, ProblemInfo};
use std::collections::HashMap;
use rand::seq::SliceRandom;

pub struct ProblemMgr{
    problems: HashMap<String, Problem>,
    problem_infos: Vec<ProblemInfo>,
    client: reqwest::Client,
}

impl ProblemMgr{
    pub async fn new() -> Self{
        let url_model = "https://kenkoooo.com/atcoder/resources/problem-models.json";
        let url_info = "https://kenkoooo.com/atcoder/resources/problems.json";
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/137.0.0.0 Safari/537.36"
            )
            .gzip(true)
            .build()
            .unwrap();

        let text_model = client
            .get(url_model)
            .header("Accept-Encoding", "gzip, deflate, br")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let text_info = client
            .get(url_info)
            .header("Accept-Encoding", "gzip, deflate, br")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        Self { 
            problems: serde_json::from_str(&text_model).unwrap(),
            problem_infos: serde_json::from_str(&text_info).unwrap(),
            client,
        }
    }

    pub fn get_problem_model_id(&self, id:&str) -> Option<&Problem>{
        self.problems.get(id)
    }

    pub fn get_problem_info_id(&self, id:&str, contest_id:&str) -> Option<&ProblemInfo>{
        self.problem_infos.iter().find(|info| info.id == id && info.contest_id == contest_id)
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
        let url_model = format!("https://atcoder.jp/contests/{}/tasks/{}", contest, id);

        let res = self.client.get(&url_model).send().await.ok()?;

        if res.status().is_success() {
            Some(url_model)
        } else {
            None
        }
    }
}
