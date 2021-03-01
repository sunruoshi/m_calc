mod util;

use std::{collections::VecDeque, convert::TryInto, time::SystemTime};
use util::{formula::Formula, utils};

fn main() {
    println!("我的口算 v0.1.0");

    println!("请输入题目数量:");
    let num_total: u32 = utils::input_number(10, 100);
    let num_range = loop {
        println!("请输入数字范围:");

        let mut range = [utils::input_number(1, 100), utils::input_number(1, 100)];

        if range[0] > range[1] {
            range.swap(0, 1);
        } else if range[0] == range[1] {
            println!("请输入不同的数字!");
            continue;
        }
        break range;
    };
    let formula_list = utils::generate_formula(num_total, num_range);
    calculate(&formula_list);
}

fn calculate(list: &VecDeque<Formula>) {
    let total: u32 = list.len().try_into().unwrap();
    let mut score: u32 = 0;
    let mut failed_list = VecDeque::new();
    let time_start = SystemTime::now();

    for formula in list {
        println!("{}", formula.get_formula());
        let answer = utils::input_number(u32::MIN, u32::MAX);
        if answer == formula.get_answer() {
            score += 1;
        } else {
            failed_list.push_back(formula);
        }
    }

    match time_start.elapsed() {
        Ok(elapsed) => {
            let time = utils::get_time(elapsed.as_secs().try_into().unwrap());
            println!("你的得分: {}分", score * 100 / total);
            println!("你的用时: {}分{}秒", time.0, time.1);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    if score != total {
        println!("错题: {}", failed_list.len());
        for formula in &failed_list {
            println!("{}", formula.get_formula());
        }
        println!("是否订正? (确定请输入y)");
        let choose = utils::input_char();
        if choose == String::from("y") {
            while failed_list.len() > 0 {
                if let Some(formula) = failed_list.pop_front() {
                    println!("{}", formula.get_formula());
                    let answer = utils::input_number(u32::MIN, u32::MAX);
                    if answer == formula.get_answer() {
                        println!("回答正确!");
                    } else {
                        failed_list.push_front(formula);
                        println!("回答错误!");
                    }
                }
            }
            println!("订正完成, 太棒了!");
        } else {
            println!("计算已结束!")
        }
    }
}
