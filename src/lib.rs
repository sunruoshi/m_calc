use chrono::{DateTime, Local};
use console::style;
use core::panic;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::Rng;
use std::{
    collections::VecDeque,
    convert::TryInto,
    env,
    error::Error,
    fs,
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
                    println!("{}", style("\n记录未找到\n是否新建? (y/n)").blue());
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

    fn add_log(&mut self, log: String) {
        self.profile = String::from(&self.profile) + &log;
    }
}

struct Formula {
    index: i32,
    pattern: i32,
    num1: i32,
    num2: i32,
}

impl Formula {
    fn get_answer(&self) -> i32 {
        match Some(self.pattern) {
            Some(0) => self.num1 + self.num2,
            Some(3) => self.num1 + self.num2,
            Some(_) => (self.num1 - self.num2).abs(),
            None => panic!("formula pattern error"),
        }
    }

    fn get_formula(&self) -> String {
        match Some(self.pattern) {
            Some(0) => format!("({}) {} + {} = ( ) ", self.index, self.num1, self.num2),
            Some(1) => format!("({}) {} - {} = ( ) ", self.index, self.num1, self.num2),
            Some(2) => format!("({}) {} - ( ) = {} ", self.index, self.num1, self.num2),
            Some(3) => format!("({}) ( ) - {} = {} ", self.index, self.num1, self.num2),
            Some(4) => format!("({}) ( ) + {} = {} ", self.index, self.num1, self.num2),
            Some(_) => format!("({}) {} + ( ) = {} ", self.index, self.num1, self.num2),
            None => panic!("formula pattern error"),
        }
    }

    fn validate(&mut self) {
        match Some(self.pattern) {
            Some(1) if self.num1 < self.num2 => self.swap_num(),
            Some(2) if self.num1 < self.num2 => self.swap_num(),
            Some(4) if self.num1 > self.num2 => self.swap_num(),
            Some(5) if self.num1 > self.num2 => self.swap_num(),
            Some(_) => (),
            None => panic!("formula pattern error"),
        }
    }

    fn new_list(user: &mut User) -> VecDeque<Formula> {
        let lv: i32 = select_level().unwrap();
        let preset: [i32; 3] = select_preset().unwrap();

        let level: i32 = if lv == 2 { 1 } else { 2 };
        let mode: String = if preset[0] == 10 {
            String::from("练习")
        } else {
            String::from("测试")
        };

        user.add_log(format!("\n[难度{} - {}]", level, mode));

        let mut formula_list: VecDeque<Formula> = VecDeque::new();

        (0..preset[0])
            .progress_with(
                ProgressBar::new(preset[0].try_into().unwrap()).with_style(
                    ProgressStyle::default_bar()
                        .template(
                            "[{bytes_per_sec:.yellow}][{bar:40.blue/red}][{percent:.yellow}%]",
                        )
                        .progress_chars("##>"),
                ),
            )
            .for_each(|i| {
                let mut formula: Formula = Formula {
                    index: i + 1,
                    pattern: rand::thread_rng().gen_range(0..lv),
                    num1: rand::thread_rng().gen_range(preset[1]..preset[2]),
                    num2: rand::thread_rng().gen_range(preset[1]..preset[2]),
                };
                formula.validate();
                formula_list.push_back(formula);
                thread::sleep(time::Duration::from_millis(10));
            });
        formula_list
    }

    fn swap_num(&mut self) {
        self.num1 ^= self.num2;
        self.num2 ^= self.num1;
        self.num1 ^= self.num2;
    }
}

pub fn select_menu(user: &mut User) -> std::io::Result<()> {
    let items: Vec<&str> = vec!["开始做题", "查看记录", "退出程序"];
    let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择:")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => {
            if let Err(e) = run(&Formula::new_list(user), user) {
                println!("Application error: {}", style(e).red());
                process::exit(1);
            }
        }
        Some(1) => {
            utils::print_profile(user);
        }
        Some(_) => {
            println!("{}", style("session end").red(),);
            process::exit(1);
        }
        None => {
            println!("{}", style("session end").red());
            process::exit(1);
        }
    }

    Ok(())
}

fn select_level() -> std::io::Result<i32> {
    let lv: i32;
    let items: Vec<&str> = vec!["难度1 (Easy)", "难度2 (Medium)"];
    let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择难度:")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => {
            lv = 2;
        }
        Some(_) => {
            lv = 6;
        }
        None => {
            println!("{}", style("session end").red());
            process::exit(1);
        }
    }

    Ok(lv)
}

fn select_preset() -> std::io::Result<[i32; 3]> {
    let preset: [i32; 3];
    let items: Vec<&str> = vec!["练习", "测试"];
    let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择模式:")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => {
            preset = [10, 1, 20];
        }
        Some(_) => {
            preset = [50, 1, 20];
        }
        None => {
            println!("{}", style("session end").red());
            process::exit(1);
        }
    }

    Ok(preset)
}

fn run(list: &VecDeque<Formula>, user: &mut User) -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();
    let time_start: time::SystemTime = time::SystemTime::now();
    let total: u32 = list.len().try_into().unwrap();
    let mut score: u32 = 0;
    let mut failed_list: VecDeque<&Formula> = VecDeque::new();
    let mut log: String = String::new();

    list.into_iter().for_each(|formula| {
        println!("{}", style(formula.get_formula()).white());
        if utils::read_number() != formula.get_answer() {
            failed_list.push_back(formula);
        } else {
            score += 1;
        }
    });

    match time_start.elapsed() {
        Ok(elapsed) => {
            let time: u32 = elapsed.as_secs().try_into().unwrap();
            log = format!(
                "\n{}\n你的得分: {}分\n你的用时: {}分{}秒\n题数: {}\n",
                now,
                score * 100 / total,
                time / 60,
                time % 60,
                total,
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
                    if utils::read_number() == formula.get_answer() {
                        println!("{}", style("回答正确!").blue());
                    } else {
                        failed_list.push_front(formula);
                        println!("{}", style("回答错误!").red());
                    }
                }
            }
            println!("{}", style("\n订正完成, 太棒了!\n").blue());
        }
        println!("{}\n", style(now).red().underlined());
    }

    user.add_log(log);

    fs::write(&(format!("./logs/{}", &user.username)), &user.profile)?;

    Ok(())
}

mod utils {
    use super::*;

    pub fn read_number() -> i32 {
        loop {
            if let Ok(value) = read_input().parse() {
                break value;
            } else {
                println!("{}", style("请输入数字!").red());
            };
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
            pattern: 0,
            num1: 1,
            num2: 5,
        };
        formula.validate();
        assert_eq!(formula.num1, 1);
    }
}
