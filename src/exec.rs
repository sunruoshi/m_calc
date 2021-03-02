use crate::lib::{utils, Formula, User};
use chrono::{DateTime, Local};
use std::{collections::VecDeque, convert::TryInto, error::Error, fs, time::SystemTime};

pub fn run(list: &VecDeque<Formula>, user: User) -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();
    let time_start = SystemTime::now();
    let total: u32 = list.len().try_into().unwrap();
    let mut score: u32 = 0;
    let mut failed_list = VecDeque::new();
    let mut log = String::new();

    for formula in list {
        println!("{}", formula.get_formula());
        if utils::read_number(u32::MIN, u32::MAX) == formula.get_answer() {
            score += 1;
        } else {
            failed_list.push_back(formula);
        }
    }

    match time_start.elapsed() {
        Ok(elapsed) => {
            let time = utils::get_time(elapsed.as_secs().try_into().unwrap());
            log = format!(
                "\n{}\n你的得分: {}分\n你的用时: {}分{}秒\n",
                now,
                score * 100 / total,
                time.0,
                time.1
            );
            println!("{}", &log);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    if score != total {
        log.push_str(&format!("错题: {}\n", failed_list.len()));
        println!("错题: {}", failed_list.len());
        for formula in &failed_list {
            log.push_str(&format!("{}\n", formula.get_formula()));
            println!("{}", formula.get_formula());
        }
        println!("是否订正? (确定请输入y)");
        if utils::read_input() == String::from("y") {
            while failed_list.len() > 0 {
                if let Some(formula) = failed_list.pop_front() {
                    println!("{}", formula.get_formula());
                    if utils::read_number(u32::MIN, u32::MAX) == formula.get_answer() {
                        println!("回答正确!");
                    } else {
                        failed_list.push_front(formula);
                        println!("回答错误!");
                    }
                }
            }
            println!("订正完成, 太棒了!\n{}", now);
        } else {
            println!("{}", now);
        }
    }

    fs::write(&user.username, user.profile + &log)?;

    Ok(())
}
