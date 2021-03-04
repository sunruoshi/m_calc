mod exec;
mod lib;

use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use lib::{utils, Formula, User};
use std::{env, process};

fn main() {
    println!("{}", style("\n我的口算 v0.1.0\n").cyan().bold());

    let mut user: User = User::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", style(err).red());
        process::exit(1);
    });

    println!(
        "{} {}\n",
        style("欢迎,").cyan().bold(),
        style(&user.username).yellow().underlined()
    );

    loop {
        select_menu(&mut user).expect("crates error");
    }
}

fn select_menu(user: &mut User) -> std::io::Result<()> {
    let items = vec!["开始做题", "查看记录", "退出程序"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择:")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => {
            if let Err(e) = exec::run(&Formula::new_list(), user) {
                println!("Application error: {}", style(e).red());
                process::exit(1);
            }
        }
        Some(1) => {
            utils::print_profile(user);
        }
        Some(2) => {
            println!("{}", style("session end").red(),);
            process::exit(1);
        }
        Some(_) => {
            println!("{}", style("请输入正确的选项!").red());
        }
        None => {
            println!("{}", style("session end").red());
            process::exit(1);
        }
    }

    Ok(())
}
