use console::style;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::Rng;
use std::{
    collections::VecDeque,
    convert::TryInto,
    env, fs,
    io::{ErrorKind, Read},
    process, thread, time,
};
pub struct User {
    pub username: String,
    pub profile: String,
}

impl User {
    pub fn new(mut args: env::Args) -> Result<User, &'static str> {
        args.next();
        let username: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Please pass a username"),
        };
        let mut file = fs::File::open(&(format!("./logs/{}", &username))).unwrap_or_else(
            |error| -> fs::File {
                if error.kind() == ErrorKind::NotFound {
                    println!("{}", style("记录未找到\n是否新建? (y/n)").blue());
                    if utils::read_input() == String::from("y") {
                        fs::File::create(&(format!("./logs/{}", &username))).unwrap_or_else(
                            |error| {
                                println!("Problem creating the file: {:?}", style(error).red());
                                process::exit(1);
                            },
                        )
                    } else {
                        println!("{}", style("session abort").red());
                        process::exit(1);
                    }
                } else {
                    println!("Problem opening the file: {:?}", style(error).red());
                    process::exit(1);
                }
            },
        );
        let mut profile: String = String::new();

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
        (0..count)
            .progress_with(
                ProgressBar::new(count.try_into().unwrap()).with_style(
                    ProgressStyle::default_bar()
                        .template(
                            "[{bytes_per_sec:.yellow}][{bar:40.blue/red}][{percent:.yellow}%]",
                        )
                        .progress_chars("##>"),
                ),
            )
            .for_each(|i| {
                let mut formula = Formula {
                    index: i + 1,
                    operator: rand::thread_rng().gen_range(0..2),
                    num1: rand::thread_rng().gen_range(range[0]..range[1]),
                    num2: rand::thread_rng().gen_range(range[0]..range[1]),
                };
                formula.validate();
                formula_list.push_back(formula);
                thread::sleep(time::Duration::from_millis(10));
            });
        formula_list
    }
    fn validate(&mut self) {
        if !(self.operator == 1 && self.num1 < self.num2) {
            return;
        }
        self.num1 ^= self.num2;
        self.num2 ^= self.num1;
        self.num1 ^= self.num2;
    }
    fn list_args() -> (u32, [u32; 2]) {
        println!("{}", style("请输入题目数量:").cyan().bold());
        (
            utils::read_number(10, 100),
            loop {
                println!("{}", style("请输入数字范围:").cyan().bold());

                let mut range = [utils::read_number(1, 100), utils::read_number(1, 100)];

                if range[0] > range[1] {
                    range.swap(0, 1);
                }
                if range[1] - range[0] < 10 {
                    println!("{}", style("数字范围至少为10").red());
                    continue;
                }
                break range;
            },
        )
    }
}

pub mod utils {
    use super::User;
    use console::style;

    pub fn read_number(low: u32, high: u32) -> u32 {
        loop {
            let num: u32 = if let Ok(value) = read_input().parse() {
                value
            } else {
                println!("{}", style("请输入数字!").red());
                continue;
            };
            if !(num < low || num > high) {
                return num;
            } else {
                println!(
                    "输入范围内的数字:{} - {}",
                    style(low).red(),
                    style(high).red()
                );
                continue;
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

    pub fn print_profile(user: &User) {
        if user.profile.len() != 0 {
            let mut count = 0;
            user.profile.lines().for_each(|line| {
                if line.contains("你的得分") {
                    count += 1;
                }
                println!("{}", style(line).white());
            });
            println!("\n共找到{}条记录\n", style(&count).red());
        } else {
            println!("{}", style("\n无记录!\n").red());
        }
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
