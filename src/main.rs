use rand::Rng;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io;
use std::mem;
use std::time::SystemTime;

fn main() {
    println!("我的口算 v0.1.0");

    println!("请输入题目数量:");
    let questions: u32 = input_number(10, 100);
    let num_range = loop {
        println!("请输入数字范围:");

        let mut range = [input_number(1, 100), input_number(1, 100)];

        if range[0] > range[1] {
            range.swap(0, 1);
        } else if range[0] == range[1] {
            println!("请输入不同的数字!");
            continue;
        }
        break range;
    };
    calculate(questions, num_range);
}

fn calculate(max_count: u32, range: [u32; 2]) {
    struct Formula {
        operator: u32,
        num1: u32,
        num2: u32,
    }

    impl Formula {
        fn check(&self, answer: u32) -> bool {
            answer
                == if self.operator == 0 {
                    self.num1 + self.num2
                } else {
                    self.num1 - self.num2
                }
        }
    }
    let weight: u32 = 100 / max_count;

    let mut count: u32 = 0;
    let mut score: u32 = 0;

    let mut failed = VecDeque::new();

    let start = SystemTime::now();

    loop {
        let mut formula = Formula {
            operator: rand::thread_rng().gen_range(0..2),
            num1: rand::thread_rng().gen_range(range[0]..range[1]),
            num2: rand::thread_rng().gen_range(range[0]..range[1]),
        };

        if formula.operator == 1 && formula.num1 < formula.num2 {
            mem::swap(&mut formula.num1, &mut formula.num2);
        }

        if formula.operator == 0 {
            println!("({}) {} + {} = __ ", count + 1, formula.num1, formula.num2);
        } else {
            println!("({}) {} - {} = __ ", count + 1, formula.num1, formula.num2);
        }

        let answer = input_number(u32::MIN, u32::MAX);

        if formula.check(answer) {
            score += 1;
        } else {
            failed.push_back(formula);
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
        for formula in &failed {
            if formula.operator == 0 {
                println!("{} + {} = __ ", formula.num1, formula.num2);
            } else {
                println!("{} - {} = __ ", formula.num1, formula.num2);
            }
        }

        println!("是否订正? (确定请输入y)");

        let mut choose = String::new();

        io::stdin().read_line(&mut choose).expect("发生了一些错误");

        if choose.trim() == "y" {
            while failed.len() > 0 {
                if let Some(formula) = failed.pop_front() {
                    if formula.operator == 0 {
                        println!("{} + {} = __ ", formula.num1, formula.num2);
                    } else {
                        println!("{} - {} = __ ", formula.num1, formula.num2);
                    }

                    let answer = input_number(u32::MIN, u32::MAX);

                    if formula.check(answer) {
                        println!("回答正确!");
                    } else {
                        failed.push_front(formula);
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

fn input_number(low: u32, high: u32) -> u32 {
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
            return valid;
        }
    }
}
