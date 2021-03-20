use chrono::{format, Local};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use prettytable::{cell, row, table, Table};
use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::TryInto,
    env,
    error::Error,
    fs,
    io::{ErrorKind, Read},
    path::Path,
    process, time,
};

pub static LOGO: &str = r#"
█▀▄▀█ █▀▀ ▄▀█ █░░ █▀▀
█░▀░█ █▄▄ █▀█ █▄▄ █▄▄
"#;

pub struct User {
    pub username: String,
    path: String,
    profile: Profile,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    records: [[(i32, String); 3]; 3],
    logs: Vec<[String; 6]>,
}

struct Formula {
    notation: String,
    answer: i32,
}

struct FormulaList {
    list: VecDeque<Formula>,
    level: i32,
    mode: String,
    range: i32,
}

impl Profile {
    fn stringify(&self) -> String {
        serde_json::to_string(self).expect("JSON Stringify Failed")
    }

    fn parse(data: String) -> Profile {
        serde_json::from_str(&data).expect("Parsing profile error")
    }
}

impl User {
    pub fn new(mut args: env::Args) -> Result<User, &'static str> {
        args.next();
        let username: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Please pass a username"),
        };
        if !Path::new("data").exists() {
            fs::create_dir("data").expect("Create Dirictory Error");
        }
        let path: String = format!("data/{}", username);
        let mut file: fs::File = fs::File::open(&path).unwrap_or_else(|e| -> fs::File {
            if e.kind() == ErrorKind::NotFound {
                println!("{}", style("\n记录未找到\n").red());
                if utils::select("是否新建").unwrap() {
                    fs::File::create(&path).unwrap_or_else(|e| {
                        println!("Problem creating the file: {:?}", style(e).red());
                        process::exit(1);
                    })
                } else {
                    println!("{}", style(LOGO).red());
                    process::exit(1);
                }
            } else {
                println!("Problem opening the file: {:?}", style(e).red());
                process::exit(1);
            }
        });

        let mut data: String = String::new();

        match file.read_to_string(&mut data).unwrap_or_else(|_| 0) {
            0 => Ok(User {
                username,
                path,
                profile: Profile {
                    records: [
                        [
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                        ],
                        [
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                        ],
                        [
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                            (i32::MAX, String::new()),
                        ],
                    ],
                    logs: Vec::new(),
                },
            }),
            _ => Ok(User {
                username,
                path,
                profile: Profile::parse(data),
            }),
        }
    }

    pub fn select(&mut self) -> std::io::Result<()> {
        match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择:")
            .items(&(vec!["开始做题", "最好成绩", "做题记录", "退出程序"]))
            .default(0)
            .interact_opt()?
        {
            Some(0) => {
                process::Command::new("clear").status().unwrap();
                if let Err(e) = self.run(&FormulaList::new().unwrap()) {
                    println!("Application error: {}", style(e).red());
                    process::exit(1);
                }
            }
            Some(1) => {
                process::Command::new("clear").status().unwrap();
                self.print_record();
                print!("\n");
            }
            Some(2) => {
                process::Command::new("clear").status().unwrap();
                self.print_logs();
            }
            Some(_) => {
                process::Command::new("clear").status().unwrap();
                println!("{}", style(LOGO).red());
                process::exit(1);
            }
            None => {
                process::Command::new("clear").status().unwrap();
                println!("{}", style("User canceled").red());
                process::exit(1);
            }
        }

        Ok(())
    }

    fn run(&mut self, this: &FormulaList) -> Result<(), Box<dyn Error>> {
        let now: format::DelayedFormat<format::StrftimeItems> = Local::now().format("%F %A %H:%M");
        let time_start: time::SystemTime = time::SystemTime::now();
        let total: u32 = this.list.len().try_into().unwrap();
        let mut score: u32 = 0;
        let mut failed_list: VecDeque<&Formula> = VecDeque::new();

        this.list.iter().for_each(|formula| {
            formula.print();
            if !formula.check() {
                failed_list.push_back(formula);
            } else {
                score += 1;
            }
        });

        match time_start.elapsed() {
            Ok(elapsed) => {
                let time: i32 = elapsed.as_secs().try_into().unwrap();
                let i: usize = (this.level - 1).try_into().unwrap();
                let j: usize = match this.range {
                    20 => 0,
                    50 => 1,
                    _ => 2,
                };
                let log: [String; 6] = [
                    format!("{}", now),
                    format!("难度{}", this.level),
                    format!("{}", this.mode),
                    format!("{}分", score * 100 / total),
                    format!("{}分{}秒", time / 60, time % 60),
                    format!("{}以内", this.range),
                ];
                process::Command::new("clear").status().unwrap();
                if this.mode == String::from("测试")
                    && score == total
                    && time < self.profile.records[i][j].0
                {
                    println!(
                        "{}",
                        style(format!("\n记录刷新! {}", Emoji("🎉🎉🎉", ":-)"))).green()
                    );
                    self.profile.records[i][j] = (time, format!("{}", now));
                }
                println!(
                    "{}",
                    style(format!("\n你的得分: {}\n你的用时: {}\n", log[3], log[4],)).green()
                );
                self.profile.logs.push(log);
            }
            Err(e) => {
                println!("Error: {:?}", style(e).red());
            }
        }

        if score != total {
            println!(
                "{} {}",
                style("错题:").red(),
                style(failed_list.len()).yellow()
            );
            failed_list.iter().for_each(|formula| {
                formula.print();
            });
            if utils::select("是否订正").unwrap() {
                while let Some(formula) = failed_list.pop_front() {
                    formula.print();
                    if formula.check() {
                        println!("{}", style("回答正确!").green());
                    } else {
                        failed_list.push_front(formula);
                        println!("{}", style("回答错误!").red());
                    }
                }
                println!("{}", style("\n订正完成!\n").green());
            }
            println!("{}\n", style(now).blue().underlined());
        }

        fs::write(&self.path, &self.profile.stringify())?;

        Ok(())
    }

    fn print_logs(&self) {
        let count: usize = self.profile.logs.len();
        if count > 0 {
            let mut table: Table =
                table!([Fg=>"做题记录", "日期", "难度", "模式", "得分", "用时", "范围"]);
            self.profile.logs.iter().enumerate().for_each(|(i, log)| {
                table.add_row(row![Fw=>
                    &format!("{}", i + 1),
                    &log[0],
                    &log[1],
                    &log[2],
                    &log[3],
                    &log[4],
                    &log[5],
                ]);
            });
            table.printstd();
        }
        println!("\n共找到{}条记录\n", style(count).red());
    }

    fn print_record(&self) {
        self.profile
            .records
            .iter()
            .enumerate()
            .for_each(|(i, record)| {
                let mut table: Table =
                    table!([Fg->&format!("难度{}", i + 1), Fw->"用时", Fw->"日期"]);
                record.iter().enumerate().for_each(|(i, v)| {
                    let range: i32 = match i {
                        0 => 20,
                        1 => 50,
                        _ => 100,
                    };
                    match Some(v.0) {
                        Some(time) if time != i32::MAX => {
                            table.add_row(row![
                                Fw->&format!("{}以内", range),
                                Fg->&format!("{}分{}秒", time / 60, time % 60),
                                Fw->&v.1,
                            ]);
                        }
                        Some(_) => {
                            table.add_row(row![Fw->&format!("{}以内", range), Fr->"无", Fr->"无"]);
                        }
                        None => (),
                    }
                });
                table.printstd();
                print!("\n");
            })
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
            notation: match key {
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

    fn print(&self) {
        println!("{}", style(&self.notation).white());
    }

    fn check(&self) -> bool {
        let answer: i32;
        loop {
            if let Ok(value) = {
                let mut input: String = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Read input error");
                input.trim().to_string()
            }
            .parse()
            {
                answer = value;
                break;
            } else {
                println!("{}", style("请输入数字!").red());
            };
        }
        answer == self.answer
    }
}

impl FormulaList {
    fn new() -> Result<FormulaList, &'static str> {
        let preset: (String, i32, i32, i32) = utils::select_preset().unwrap();
        let (level, mode, range) = (
            utils::select_level().unwrap(),
            String::from(&preset.0),
            preset.3,
        );
        let mut list: VecDeque<Formula> = VecDeque::new();

        let bar: ProgressBar = ProgressBar::new(preset.1.try_into().unwrap()).with_style(
            ProgressStyle::default_bar()
                .template("{prefix}[{bar:40.blue/red}][{pos:.yellow}/{len:.yellow}]")
                .progress_chars("##>"),
        );

        bar.println("\n");
        bar.set_prefix(&format!("{}", Emoji("🚚 ", ":-)")));
        (0..preset.1).progress_with(bar).for_each(|i| {
            list.push_back(
                Formula::new([i + 1, level, preset.2, preset.3]).unwrap_or_else(|e| {
                    println!("Error: {:?}", style(e).red());
                    process::exit(1);
                }),
            );
        });

        Ok(FormulaList {
            list,
            level,
            mode,
            range,
        })
    }
}

mod utils {
    use super::*;

    fn select_max_num() -> std::io::Result<i32> {
        Ok(
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择数字范围:")
                .items(&(vec!["20以内", "50以内", "100以内"]))
                .default(0)
                .interact_opt()?
            {
                Some(0) => 20,
                Some(1) => 50,
                Some(_) => 100,
                None => {
                    println!("{}", style("User canceled").red());
                    process::exit(1);
                }
            },
        )
    }

    pub fn select_level() -> std::io::Result<i32> {
        Ok(
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择难度:")
                .items(&(vec!["难度1 (Easy)", "难度2 (Medium)", "难度3 (Hard)"]))
                .default(0)
                .interact_opt()?
            {
                Some(0) => 1,
                Some(1) => 2,
                Some(_) => 3,
                None => {
                    println!("{}", style("User canceled").red());
                    process::exit(1);
                }
            },
        )
    }

    pub fn select_preset() -> std::io::Result<(String, i32, i32, i32)> {
        Ok(
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择模式:")
                .items(&(vec!["练习", "测试"]))
                .default(0)
                .interact_opt()?
            {
                Some(0) => (String::from("练习"), 10, 1, select_max_num().unwrap()),
                Some(_) => (String::from("测试"), 50, 1, select_max_num().unwrap()),
                None => {
                    println!("{}", style("User canceled").red());
                    process::exit(1);
                }
            },
        )
    }

    pub fn select(prompt: &str) -> std::io::Result<bool> {
        Ok(
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .items(&(vec!["Yes", "No"]))
                .default(0)
                .interact_opt()?
            {
                Some(0) => true,
                Some(_) => false,
                None => {
                    println!("{}", style("User canceled").red());
                    process::exit(1);
                }
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ternary_formula() {
        let this: FormulaList = FormulaList::new().unwrap();
        this.list.iter().for_each(|i| {
            println!("{} [{}]", i.notation, i.answer);
        });
    }
}
