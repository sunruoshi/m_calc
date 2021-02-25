use rand::Rng;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io;
use std::time::SystemTime;

fn main() {
    println!("我的口算 v0.1.0");

    println!("请输入题目数量:");
    let questions: u32 = input_number(1, 100);
    println!("请输入数字范围:");
    let range: u32 = input_number(10, 100);
    calculate(questions, range);
}

fn calculate(max_count: u32, num_range: u32) {
    let weight: u32 = 100 / max_count;

    let mut count: u32 = 0;
    let mut score: u32 = 0;

    let mut failed = Vec::new();

    let start = SystemTime::now();

    loop {
        let num1 = rand::thread_rng().gen_range(1..num_range);
        let num2 = rand::thread_rng().gen_range(1..num_range);
        let operater = rand::thread_rng().gen_range(0..2);

        let big: u32;
        let small: u32;

        match num1.cmp(&num2) {
            Ordering::Less => {
                big = num2;
                small = num1;
            }
            Ordering::Greater => {
                big = num1;
                small = num2;
            }
            Ordering::Equal => {
                big = num1;
                small = num2;
            }
        }

        if operater == 0 {
            println!("({}) {} + {} = __ ", count + 1, big, small);
        } else {
            println!("({}) {} - {} = __ ", count + 1, big, small);
        }

        let answer = input_number(u32::MIN, u32::MAX);

        if answer_check(answer, operater, big, small) {
            score += 1;
        } else {
            failed.push([big, small, operater]);
        }

        count += 1;

        if count == max_count {
            match start.elapsed() {
                Ok(elapsed) => {
                    let time = time_format(elapsed.as_secs().try_into().unwrap());
                    println!("你的得分: {}分", score * weight);
                    println!("你的用时: {}分{}秒", time[0], time[1]);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            break;
        }
    }
    if score != max_count {
        println!("错题: {}", failed.len());
        for [bg, sm, op] in &failed {
            if op == &0 {
                println!("{} + {} = __ ", bg, sm);
            } else {
                println!("{} - {} = __ ", bg, sm);
            }
        }

        println!("是否订正? (确定请输入y)");

        let mut choose = String::new();

        io::stdin().read_line(&mut choose).expect("发生了一些错误");

        if choose.trim() == "y" {
            while failed.len() > 0 {
                if let Some(nums) = failed.pop() {
                    if nums[2] == 0 {
                        println!("{} + {} = __ ", nums[0], nums[1]);
                    } else {
                        println!("{} - {} = __ ", nums[0], nums[1]);
                    }

                    let answer = input_number(u32::MIN, u32::MAX);

                    if answer_check(answer, nums[2], nums[0], nums[1]) {
                        println!("回答正确!");
                    } else {
                        failed.push([nums[0], nums[1], nums[2]]);
                        println!("回答错误!");
                    }
                }
            }
            println!("订正完成, 太棒了!");
        } else {
            println!("程序已关闭")
        }
    }
}

fn time_format(time: u32) -> Vec<u32> {
    [time / 60, time % 60].to_vec()
}

fn answer_check(answer: u32, op: u32, bg: u32, sm: u32) -> bool {
    let correct_answer: u32;
    if op == 0 {
        correct_answer = bg + sm;
    } else {
        correct_answer = bg - sm;
    };
    answer == correct_answer
}

fn input_number(low: u32, high: u32) -> u32 {
    let output: u32;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("发生了一些错误");

        let valid: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("请输入数字!");
                continue;
            }
        };
        if valid < low || valid > high {
            println!("输入范围内的数字:{} - {}", low, high);
            continue;
        } else {
            output = valid;
            break;
        }
    }
    output
}
