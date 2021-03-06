use chrono::{DateTime, Local};
use console::style;
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
    process, time,
};
pub struct User {
    pub username: String,
    pub profile: String,
}

struct Formula {
    pattern: String,
    answer: i32,
}

struct FormulaList {
    list: VecDeque<Formula>,
    level: i32,
    mode: String,
}

impl User {
    pub fn new(mut args: env::Args) -> Result<User, &'static str> {
        args.next();
        let username: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Please pass a username"),
        };
        let mut file =
            fs::File::open(&(format!("{}", &username))).unwrap_or_else(|error| -> fs::File {
                if error.kind() == ErrorKind::NotFound {
                    println!("{}", style("\n记录未找到\n是否新建? (y/n)").blue());
                    if utils::read_input() == String::from("y") {
                        fs::File::create(&(format!("{}", &username))).unwrap_or_else(|error| {
                            println!("Problem creating the file: {:?}", style(error).red());
                            process::exit(1);
                        })
                    } else {
                        println!("{}", style("session abort").red());
                        process::exit(1);
                    }
                } else {
                    println!("Problem opening the file: {:?}", style(error).red());
                    process::exit(1);
                }
            });
        let mut profile: String = String::new();

        file.read_to_string(&mut profile).unwrap_or_else(|_| 0);

        Ok(User { username, profile })
    }

    pub fn select(&mut self) -> std::io::Result<()> {
        let items: Vec<&str> = vec!["开始做题", "查看记录", "退出程序"];
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择:")
            .items(&items)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(0) => {
                if let Err(e) = self.run(&FormulaList::new().unwrap()) {
                    println!("Application error: {}", style(e).red());
                    process::exit(1);
                }
            }
            Some(1) => {
                self.print_profile();
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

    fn run(&mut self, this: &FormulaList) -> Result<(), Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        let time_start: time::SystemTime = time::SystemTime::now();
        let total: u32 = this.list.len().try_into().unwrap();
        let mut score: u32 = 0;
        let mut failed_list: VecDeque<&Formula> = VecDeque::new();
        let mut log: String = format!("\n[难度{} - {}]", this.level, this.mode);

        this.list.iter().for_each(|formula| {
            formula.print_pattern();
            if utils::read_number() != formula.answer {
                failed_list.push_back(formula);
            } else {
                score += 1;
            }
        });

        match time_start.elapsed() {
            Ok(elapsed) => {
                let time: u32 = elapsed.as_secs().try_into().unwrap();
                log.push_str(&format!(
                    "\n{}\n你的得分: {}分\n你的用时: {}分{}秒\n题数: {}\n",
                    now,
                    score * 100 / total,
                    time / 60,
                    time % 60,
                    total,
                ));
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
                log.push_str(&format!("{}\n", formula.pattern));
                formula.print_pattern();
            });
            println!("{}", style("是否订正? (y/n)").blue());
            if utils::read_input() == String::from("y") {
                while failed_list.len() > 0 {
                    if let Some(formula) = failed_list.pop_front() {
                        println!("{}", style(&formula.pattern).white());
                        if utils::read_number() == formula.answer {
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

        self.add_log(log);

        fs::write(&(format!("{}", self.username)), &self.profile)?;

        Ok(())
    }

    fn print_profile(&self) {
        if self.profile.len() != 0 {
            let mut count = 0;
            self.profile.lines().for_each(|line| {
                if line.contains('[') {
                    count += 1;
                }
                println!("{}", style(line).white());
            });
            println!("\n共找到{}条记录\n", style(&count).red());
        } else {
            println!("{}", style("\n无记录!\n").red());
        }
    }

    fn add_log(&mut self, log: String) {
        self.profile = String::from(&self.profile) + &log;
    }
}

impl Formula {
    fn new(args: [i32; 4]) -> Result<Formula, &'static str> {
        let index: i32 = args[0];
        let mut nums: [i32; 2] = [args[2], args[3]];

        match Some(args[1]) {
            Some(1) if nums[0] < nums[1] => nums.swap(0, 1),
            Some(2) if nums[0] < nums[1] => nums.swap(0, 1),
            Some(4) if nums[0] > nums[1] => nums.swap(0, 1),
            Some(5) if nums[0] > nums[1] => nums.swap(0, 1),
            Some(_) => (),
            None => return Err("Failed to parse nums"),
        }

        Ok(Formula {
            pattern: match Some(args[1]) {
                Some(0) => format!("({}) {} + {} = ( )", index, nums[0], nums[1]),
                Some(1) => format!("({}) {} - {} = ( )", index, nums[0], nums[1]),
                Some(2) => format!("({}) {} - ( ) = {}", index, nums[0], nums[1]),
                Some(3) => format!("({}) ( ) - {} = {}", index, nums[0], nums[1]),
                Some(4) => format!("({}) ( ) + {} = {}", index, nums[0], nums[1]),
                Some(_) => format!("({}) {} + ( ) = {}", index, nums[0], nums[1]),
                None => return Err("Failed to parse pattern"),
            },
            answer: match Some(args[1]) {
                Some(0) => nums[0] + nums[1],
                Some(3) => nums[0] + nums[1],
                Some(_) => (nums[0] - nums[1]).abs(),
                None => return Err("Failed to parse answer"),
            },
        })
    }

    fn print_pattern(&self) {
        println!("{}", style(&self.pattern).white());
    }
}

impl FormulaList {
    fn new() -> Result<FormulaList, &'static str> {
        let lv: i32 = utils::select_level().unwrap();
        let preset: [i32; 3] = utils::select_preset().unwrap();

        let level: i32 = if lv == 2 { 1 } else { 2 };
        let mode: String = if preset[0] == 10 {
            String::from("练习")
        } else {
            String::from("测试")
        };

        let mut list: VecDeque<Formula> = VecDeque::new();

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
                let formula: Formula = Formula::new([
                    i + 1,
                    rand::thread_rng().gen_range(0..lv),
                    rand::thread_rng().gen_range(preset[1]..preset[2]),
                    rand::thread_rng().gen_range(preset[1]..preset[2]),
                ])
                .unwrap_or_else(|error| {
                    println!("Error: {:?}", style(error).red());
                    process::exit(1);
                });
                list.push_back(formula);
            });

        Ok(FormulaList { list, level, mode })
    }
}

mod utils {
    use super::*;

    pub fn select_level() -> std::io::Result<i32> {
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

    pub fn select_preset() -> std::io::Result<[i32; 3]> {
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
}

#[cfg(test)]
mod test {
    #[test]
    fn formula_validate() {}
}
