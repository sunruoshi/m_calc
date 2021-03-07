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
                    println!("{}", style("\n记录未找到\n").red());
                    if utils::select("是否新建").unwrap() {
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
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择:")
            .items(&(vec!["开始做题", "查看记录", "退出程序"]))
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
                println!("{}", style("Process exit").red(),);
                process::exit(1);
            }
            None => {
                println!("{}", style("User canceled").red());
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
            if utils::select("是否订正").unwrap() {
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
        match Some(self.profile.len()) {
            Some(0) => println!("{}", style("\n无记录!\n").red()),
            Some(_) => {
                let mut count: i32 = 0;
                self.profile.lines().for_each(|line| {
                    if line.contains('[') {
                        count += 1;
                    }
                    println!("{}", style(line).white());
                });
                println!("\n共找到{}条记录\n", style(&count).red());
            }
            None => (),
        }
    }

    fn add_log(&mut self, log: String) {
        self.profile = String::from(&self.profile) + &log;
    }
}

impl Formula {
    fn new(args: [i32; 4]) -> Result<Formula, &'static str> {
        let idx: i32 = args[0];
        let key: i32 = match Some(args[1]) {
            Some(1) => rand::thread_rng().gen_range(0..2),
            Some(2) => rand::thread_rng().gen_range(0..6),
            Some(_) => rand::thread_rng().gen_range(6..10),
            None => return Err("Failed to generate rand key"),
        };
        let mut nums: [i32; 3] = [
            rand::thread_rng().gen_range(args[2]..args[3]),
            rand::thread_rng().gen_range(args[2]..args[3]),
            rand::thread_rng().gen_range(args[2]..args[3]),
        ];

        match Some(key) {
            Some(1) if nums[0] < nums[1] => nums.swap(0, 1),
            Some(2) if nums[0] < nums[1] => nums.swap(0, 1),
            Some(4) if nums[0] > nums[1] => nums.swap(0, 1),
            Some(5) if nums[0] > nums[1] => nums.swap(0, 1),
            Some(7) if nums[0] + nums[1] - nums[2] < 0 => nums.swap(1, 2),
            Some(8) if nums[0] - nums[1] + nums[2] < 0 => nums.swap(0, 1),
            Some(9) => {
                while nums[0] - nums[1] - nums[2] < 0 {
                    nums = [
                        rand::thread_rng().gen_range(args[2]..args[3]),
                        rand::thread_rng().gen_range(args[2]..args[3]),
                        rand::thread_rng().gen_range(args[2]..args[3]),
                    ]
                }
            }
            Some(_) => (),
            None => return Err("Failed to validate nums"),
        }

        Ok(Formula {
            pattern: match Some(key) {
                Some(0) => format!("({}) {} + {} = ( )", idx, nums[0], nums[1]),
                Some(1) => format!("({}) {} - {} = ( )", idx, nums[0], nums[1]),
                Some(2) => format!("({}) {} - ( ) = {}", idx, nums[0], nums[1]),
                Some(3) => format!("({}) ( ) - {} = {}", idx, nums[0], nums[1]),
                Some(4) => format!("({}) ( ) + {} = {}", idx, nums[0], nums[1]),
                Some(5) => format!("({}) {} + ( ) = {}", idx, nums[0], nums[1]),
                Some(6) => format!("({}) {} + {} + {} = ( )", idx, nums[0], nums[1], nums[2]),
                Some(7) => format!("({}) {} + {} - {} = ( )", idx, nums[0], nums[1], nums[2]),
                Some(8) => format!("({}) {} - {} + {} = ( )", idx, nums[0], nums[1], nums[2]),
                Some(_) => format!("({}) {} - {} - {} = ( )", idx, nums[0], nums[1], nums[2]),
                None => return Err("Failed to parse pattern"),
            },
            answer: match Some(key) {
                Some(0) => nums[0] + nums[1],
                Some(3) => nums[0] + nums[1],
                Some(6) => nums[0] + nums[1] + nums[2],
                Some(7) => nums[0] + nums[1] - nums[2],
                Some(8) => nums[0] - nums[1] + nums[2],
                Some(9) => nums[0] - nums[1] - nums[2],
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
        let level: i32 = utils::select_level().unwrap();
        let preset: (String, i32, i32, i32) = utils::select_preset().unwrap();
        let mut list: VecDeque<Formula> = VecDeque::new();

        (0..preset.1)
            .progress_with(
                ProgressBar::new(preset.1.try_into().unwrap()).with_style(
                    ProgressStyle::default_bar()
                        .template(
                            "[{bytes_per_sec:.yellow}][{bar:40.blue/red}][{percent:.yellow}%]",
                        )
                        .progress_chars("##>"),
                ),
            )
            .for_each(|i| {
                let formula: Formula = Formula::new([i + 1, level, preset.2, preset.3])
                    .unwrap_or_else(|error| {
                        println!("Error: {:?}", style(error).red());
                        process::exit(1);
                    });
                list.push_back(formula);
            });

        Ok(FormulaList {
            list,
            level,
            mode: preset.0,
        })
    }
}

mod utils {
    use super::*;

    pub fn select_level() -> std::io::Result<i32> {
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择难度:")
            .items(&(vec!["难度1 (Easy)", "难度2 (Medium)", "难度3 (Hard)"]))
            .default(0)
            .interact_opt()?;

        let lv: i32 = match selection {
            Some(0) => 1,
            Some(1) => 2,
            Some(_) => 3,
            None => {
                println!("{}", style("User canceled").red());
                process::exit(1);
            }
        };

        Ok(lv)
    }

    pub fn select_preset() -> std::io::Result<(String, i32, i32, i32)> {
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择模式:")
            .items(&(vec!["练习", "测试"]))
            .default(0)
            .interact_opt()?;

        let preset: (String, i32, i32, i32) = match selection {
            Some(0) => (String::from("练习"), 10, 1, 20),
            Some(_) => (String::from("测试"), 50, 1, 20),
            None => {
                println!("{}", style("User canceled").red());
                process::exit(1);
            }
        };

        Ok(preset)
    }

    pub fn select(prompt: &str) -> std::io::Result<bool> {
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&(vec!["Yes", "No"]))
            .default(0)
            .interact_opt()?;

        let choose: bool = match selection {
            Some(0) => true,
            Some(_) => false,
            None => {
                println!("{}", style("User canceled").red());
                process::exit(1);
            }
        };

        Ok(choose)
    }

    pub fn read_number() -> i32 {
        loop {
            if let Ok(value) = {
                let mut input: String = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Some error occurred");
                input.trim().to_string()
            }
            .parse()
            {
                break value;
            } else {
                println!("{}", style("请输入数字!").red());
            };
        }
    }
}

#[cfg(test)]
mod test {
    use std::time;

    use super::Formula;

    #[test]
    fn test_ternary_formula() {
        let time_start: time::SystemTime = time::SystemTime::now();
        (0..100).for_each(|i| {
            let formula: Formula = Formula::new([i + 1, 3, 1, 20]).unwrap();
            println!("{} [{}]", formula.pattern, formula.answer);
        });
        match time_start.elapsed() {
            Ok(elapsed) => {
                let time: u128 = elapsed.as_millis();
                println!("{}", time);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
