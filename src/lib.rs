use rand::Rng;
use std::io::{ErrorKind, Read};
use std::{collections::VecDeque, fs, process};

pub struct User {
    pub username: String,
    pub profile: String,
}

impl User {
    pub fn new(args: &[String]) -> Result<User, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let username = args[1].clone();
        let mut file = fs::File::open(&username.as_str()).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                fs::File::create(&username.as_str()).unwrap_or_else(|error| {
                    println!("Problem creating the file: {:?}", error);
                    process::exit(1);
                })
            } else {
                println!("Problem opening the file: {:?}", error);
                process::exit(1);
            }
        });
        let mut profile = String::new();

        file.read_to_string(&mut profile).unwrap_or_else(|_| 0);

        Ok(User { username, profile })
    }
}

pub struct Formula {
    pub index: u32,
    pub operator: u32,
    pub num1: u32,
    pub num2: u32,
}

impl Formula {
    pub fn get_answer(&self) -> u32 {
        return if self.operator == 0 {
            self.num1 + self.num2
        } else {
            self.num1 - self.num2
        };
    }
    pub fn get_formula(&self) -> String {
        return if self.operator == 0 {
            format!("({}) {} + {} = ( ) ", self.index, self.num1, self.num2)
        } else {
            format!("({}) {} - {} = ( ) ", self.index, self.num1, self.num2)
        };
    }
    pub fn new_list() -> VecDeque<Formula> {
        let mut formula_list: VecDeque<Formula> = VecDeque::new();
        let (count, range) = Formula::list_args();
        for i in 1..count + 1 {
            let mut formula = Formula {
                index: i,
                operator: rand::thread_rng().gen_range(0..2),
                num1: rand::thread_rng().gen_range(range[0]..range[1]),
                num2: rand::thread_rng().gen_range(range[0]..range[1]),
            };
            formula.validate();
            formula_list.push_back(formula);
        }
        formula_list
    }
    fn validate(&mut self) {
        if self.operator == 1 && self.num1 < self.num2 {
            self.num1 ^= self.num2;
            self.num2 ^= self.num1;
            self.num1 ^= self.num2;
        }
    }
    fn list_args() -> (u32, [u32; 2]) {
        println!("请输入题目数量:");
        let num_total: u32 = utils::read_number(10, 100);
        let num_range = loop {
            println!("请输入数字范围:");

            let mut range = [utils::read_number(1, 100), utils::read_number(1, 100)];

            if range[0] > range[1] {
                range.swap(0, 1);
            } else if range[0] == range[1] {
                println!("请输入不同的数字!");
                continue;
            }
            break range;
        };
        (num_total, num_range)
    }
}

pub mod utils {
    pub fn read_number(low: u32, high: u32) -> u32 {
        loop {
            let num: u32 = match read_input().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("请输入数字!");
                    continue;
                }
            };
            if num < low || num > high {
                println!("输入范围内的数字:{} - {}", low, high);
                continue;
            } else {
                return num;
            }
        }
    }

    pub fn read_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Some error occurred");
        input.trim().to_string()
    }

    pub fn get_time(time: u32) -> (u32, u32) {
        (time / 60, time % 60)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formula_validate() {
        let mut formula = Formula {
            index: 1,
            operator: 0,
            num1: 1,
            num2: 5,
        };
        formula.validate();
        assert_eq!(formula.num1, 1);
    }
}