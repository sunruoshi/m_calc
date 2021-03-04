use crate::lib::{utils, Formula, User};
use chrono::{DateTime, Local};
use console::style;
use std::{collections::VecDeque, convert::TryInto, error::Error, fs, time::SystemTime};

pub fn run(list: &VecDeque<Formula>, user: &mut User) -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();
    let time_start: SystemTime = SystemTime::now();
    let total: u32 = list.len().try_into().unwrap();
    let mut score: u32 = 0;
    let mut failed_list: VecDeque<&Formula> = VecDeque::new();
    let mut log: String = String::new();

    list.into_iter().for_each(|formula| {
        println!("{}", style(formula.get_formula()).white());
        if utils::read_number(u32::MIN, u32::MAX) != formula.get_answer() {
            failed_list.push_back(formula);
        } else {
            score += 1;
        }
    });

    match time_start.elapsed() {
        Ok(elapsed) => {
            let time: u32 = elapsed.as_secs().try_into().unwrap();
            log = format!(
                "\n{}\n你的得分: {}分\n你的用时: {}分{}秒\n",
                now,
                score * 100 / total,
                time / 60,
                time % 60,
            );
            println!("{}", style(&log).yellow());
        }
        Err(e) => {
            println!("Error: {:?}", style(e).red());
        }
    }

    if score != total {
        log.push_str(&format!("错题: {}\n", failed_list.len()));
        println!(
            "{} {}",
            style("错题:").red(),
            style(failed_list.len()).yellow()
        );
        failed_list.iter().for_each(|formula| {
            log.push_str(&format!("{}\n", formula.get_formula()));
            println!("{}", style(formula.get_formula()).white());
        });
        println!("{}", style("是否订正? (y/n)").blue());
        if utils::read_input() == String::from("y") {
            while failed_list.len() > 0 {
                if let Some(formula) = failed_list.pop_front() {
                    println!("{}", style(formula.get_formula()).white());
                    if utils::read_number(u32::MIN, u32::MAX) == formula.get_answer() {
                        println!("{}", style("回答正确!").blue());
                    } else {
                        failed_list.push_front(formula);
                        println!("{}", style("回答错误!").red());
                    }
                }
            }
            println!("{}", style("\n订正完成, 太棒了!\n").blue());
        }
        println!("{}", style(now).red().underlined());
    }

    user.profile = String::from(&user.profile) + &log;

    fs::write(&(format!("./logs/{}", &user.username)), &user.profile)?;

    Ok(())
}
