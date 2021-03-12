use chrono::{DateTime, Local};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::TryInto,
    env,
    error::Error,
    fs,
    io::{ErrorKind, Read},
    process, time, usize,
};
pub struct User {
    pub username: String,
    pub profile: Profile,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    name: String,
    record: Vec<i32>,
    logs: String,
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
        let mut file: fs::File =
            fs::File::open(&String::from(&username)).unwrap_or_else(|e| -> fs::File {
                if e.kind() == ErrorKind::NotFound {
                    println!("{}", style("\n记录未找到\n").red());
                    if utils::select("是否新建").unwrap() {
                        fs::File::create(&String::from(&username)).unwrap_or_else(|e| {
                            println!("Problem creating the file: {:?}", style(e).red());
                            process::exit(1);
                        })
                    } else {
                        println!("{}", style("session abort").red());
                        process::exit(1);
                    }
                } else {
                    println!("Problem opening the file: {:?}", style(e).red());
                    process::exit(1);
                }
            });

        let mut data: String = String::new();

        file.read_to_string(&mut data).unwrap_or_else(|_| 0);

        let mut profile: Profile = Profile {
            name: String::from(&username),
            record: vec![i32::MAX, i32::MAX, i32::MAX],
            logs: String::new(),
        };

        if data.len() != 0 {
            profile = serde_json::from_str(&data).expect("Init user profile error")
        }

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
                println!("{}", style("Process exit").red());
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
                let time: i32 = elapsed.as_secs().try_into().unwrap();
                let idx: usize = (this.level - 1).try_into().unwrap();
                if this.mode == String::from("测试")
                    && score == total
                    && time < self.profile.record[idx]
                {
                    println!("{}", style("\n记录刷新!").green().underlined());
                    self.profile.record[idx] = time;
                }
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
                while let Some(formula) = failed_list.pop_front() {
                    println!("{}", style(&formula.pattern).white());
                    if utils::read_number() == formula.answer {
                        println!("{}", style("回答正确!").green());
                    } else {
                        failed_list.push_front(formula);
                        println!("{}", style("回答错误!").red());
                    }
                }
                println!("{}", style("\n订正完成, 太棒了!\n").green());
            }
            println!("{}\n", style(now).blue().underlined());
        }

        self.add_log(log);

        fs::write(
            &(format!("{}", self.username)),
            serde_json::to_string(&self.profile).expect("JSON Stringify Failed"),
        )?;

        Ok(())
    }

    fn print_profile(&self) {
        let mut count: i32 = 0;
        self.profile.logs.lines().for_each(|line| {
            if line.contains('[') {
                count += 1;
            }
            println!("{}", style(line).white());
        });
        println!(
            "\n共找到{}条记录\n\n{}",
            style(count).red(),
            style("最好成绩:").green()
        );
        (0..self.profile.record.len()).for_each(|i| match Some(self.profile.record[i]) {
            Some(v) if v != i32::MAX => print!("难度{}: {}分{}秒\n", i + 1, v / 60, v % 60),
            Some(_) => print!("难度{}: 无\n", i + 1),
            None => (),
        });
        print!("\n");
    }

    fn add_log(&mut self, log: String) {
        self.profile.logs = String::from(&self.profile.logs) + &log;
    }
}

impl Formula {
    fn new(args: [i32; 4]) -> Result<Formula, &'static str> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let range: Uniform<i32> = Uniform::from(args[2]..args[3]);
        let idx: i32 = args[0];
        let (key, len): (i32, usize) = match args[1] {
            1 => (rng.gen_range(0..2), 2),
            2 => (rng.gen_range(2..6), 2),
            _ => (rng.gen_range(6..10), 3),
        };
        let mut nums: Vec<i32> = (&mut rng).sample_iter(&range).take(len).collect();

        match key {
            1 | 2 if nums[0] < nums[1] => nums.swap(0, 1),
            4 | 5 if nums[0] > nums[1] => nums.swap(0, 1),
            7 if nums[0] + nums[1] - nums[2] < 0 => nums.swap(1, 2),
            8 if nums[0] - nums[1] + nums[2] < 0 => nums.swap(0, 1),
            9 => {
                while nums[0] - nums[1] - nums[2] < 0 {
                    nums = (&mut rng).sample_iter(&range).take(len).collect()
                }
            }
            _ => (),
        }

        Ok(Formula {
            pattern: match key {
                0 => format!("({}) {} + {} = ( )", idx, nums[0], nums[1]),
                1 => format!("({}) {} - {} = ( )", idx, nums[0], nums[1]),
                2 => format!("({}) {} - ( ) = {}", idx, nums[0], nums[1]),
                3 => format!("({}) ( ) - {} = {}", idx, nums[0], nums[1]),
                4 => format!("({}) ( ) + {} = {}", idx, nums[0], nums[1]),
                5 => format!("({}) {} + ( ) = {}", idx, nums[0], nums[1]),
                6 => format!("({}) {} + {} + {} = ( )", idx, nums[0], nums[1], nums[2]),
                7 => format!("({}) {} + {} - {} = ( )", idx, nums[0], nums[1], nums[2]),
                8 => format!("({}) {} - {} + {} = ( )", idx, nums[0], nums[1], nums[2]),
                _ => format!("({}) {} - {} - {} = ( )", idx, nums[0], nums[1], nums[2]),
            },
            answer: match key {
                0 | 3 => nums[0] + nums[1],
                6 => nums[0] + nums[1] + nums[2],
                7 => nums[0] + nums[1] - nums[2],
                8 => nums[0] - nums[1] + nums[2],
                9 => nums[0] - nums[1] - nums[2],
                _ => (nums[0] - nums[1]).abs(),
            },
        })
    }

    fn print_pattern(&self) {
        println!("{}", style(&self.pattern).white());
    }
}

impl FormulaList {
    fn new() -> Result<FormulaList, &'static str> {
        let preset: (String, i32, i32, i32) = utils::select_preset().unwrap();
        let (level, mode) = (utils::select_level().unwrap(), String::from(&preset.0));
        let mut list: VecDeque<Formula> = VecDeque::new();

        let bar: ProgressBar = ProgressBar::new(preset.1.try_into().unwrap()).with_style(
            ProgressStyle::default_bar()
                .template("{prefix}[{bar:40.blue/red}][{pos:.yellow}/{len:.yellow}]")
                .progress_chars("##>"),
        );

        bar.println("\n");
        bar.set_prefix(&format!("{}", Emoji("🚚 ", ":-)")));
        (0..preset.1).progress_with(bar).for_each(|i| {
            let formula: Formula =
                Formula::new([i + 1, level, preset.2, preset.3]).unwrap_or_else(|e| {
                    println!("Error: {:?}", style(e).red());
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
    use super::*;

    #[test]
    fn test_ternary_formula() {
        let this: FormulaList = FormulaList::new().unwrap();
        this.list.iter().for_each(|i| {
            println!("{} [{}]", i.pattern, i.answer);
        });
    }
}
