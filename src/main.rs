//main.rs

mod commands;
mod problem_mgr;
//use serde_json;

use commands::Command;

//use crate::problem_mgr::problem::Problem;
use crate::problem_mgr::ProblemMgr;
//use crate::Command;

use chrono::Local;
use inquire::Text;
use std::thread;
use std::time::Duration;
//use std::io;

#[tokio::main]
async fn main() {
    let problem_mgr = ProblemMgr::new().await;

    loop {
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
                    problem_id = problem_mgr
                        .get_problem_random_min_max(min_diff, max_diff)
                        .unwrap();

                    match problem_mgr.get_problem_url(&problem_id).await {
                        Some(state) => {
                            url = state.clone();
                            break;
                        }
                        None => {
                            continue;
                        }
                    }
                }

                let problem = problem_mgr.get_problem_model_id(&problem_id).unwrap();
                println!("Ready...");
                thread::sleep(Duration::from_secs_f32(1.5f32));

                println!("-----------------------------------------------------");
                println!("contest:      {}", &problem_id.split('_').next().unwrap());
                println!(
                    "title:        {}",
                    problem_mgr
                        .get_problem_info_id(&problem_id, &problem_id.split('_').next().unwrap())
                        .unwrap()
                        .title
                );
                //println!("{}", )
                println!("link:         {}", url);
                println!("difficulty:   {}", problem.difficulty.unwrap());
                println!("-----------------------------------------------------");

                let begin_time: i64 = Local::now().timestamp();
                let _ = Text::new("If you have finish this problem, Pease Enter.").prompt();
                //println!("{:?}", problem_mgr.get_problem_random_min_max(min_diff, max_diff).unwrap());
                let clear_time = Local::now().timestamp() - begin_time;
                let clear_time_str = format!(
                    "[{:0>2}:{:0>2}:{:0>2}]",
                    clear_time / 3600,
                    (clear_time / 60) % 60,
                    clear_time % 60
                );
                println!("Clear time {}", clear_time_str);
                //let _ = io::stdin().read_line(&mut String::new());
                //let _ = Text::new("").prompt();
            }
            Command::Exit => {
                break;
            }
        }
    }

    println!("See you.");
    //println!("{:?}", problem_mgr.await.get_problem_id("abc460_d").unwrap());
}
