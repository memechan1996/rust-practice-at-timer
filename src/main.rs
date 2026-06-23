//main.rs

mod problem_mgr;
mod commands;
//use serde_json;

use commands::Command;

//use crate::problem_mgr::problem::Problem;
use crate::problem_mgr::{ProblemMgr, problem};
//use crate::Command;

use inquire::Text;

#[tokio::main]
async fn main(){
    let mut problem_mgr = ProblemMgr::new().await;

    loop{
        let command = Command::select("").prompt().unwrap();

        match command {
            Command::Start => {
                let input = Text::new("Min difficulty:").prompt().unwrap();
                let min_diff: f32 = input.parse().unwrap();
                let input = Text::new("Max difficulty:").prompt().unwrap();
                let max_diff: f32 = input.parse().unwrap();

                let url: String;
                let mut problem_id: String;
                loop {
                    problem_id = problem_mgr.get_problem_random_min_max(min_diff, max_diff).unwrap();

                    match problem_mgr
                        .get_problem_url(&problem_id)
                        .await
                    {
                        Some(state) => {
                            url = state.clone();
                            break;
                        },
                        None => {continue;},
                    }
                }

                let problem = problem_mgr.get_problem_id(&problem_id).unwrap();

                println!("----------------------");
                //println!("{}", )
                println!("{}", url);
                //println!("{:?}", problem_mgr.get_problem_random_min_max(min_diff, max_diff).unwrap());
            },
            Command::Exit => {
                break;
            }
        }
    }

    println!("See you.");
    //println!("{:?}", problem_mgr.await.get_problem_id("abc460_d").unwrap());
}
