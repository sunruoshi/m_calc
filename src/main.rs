mod exec;
mod lib;

use lib::{utils, Formula, User};
use std::{env, process};

fn main() {
    println!("我的口算 v0.1.0");

    let args: Vec<String> = env::args().collect();

    let mut user = User::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("欢迎, {}", user.username);

    loop {
        println!("\n请选择:\n1->开始做题 2->查看记录 0->退出程序");

        let choose: String = utils::read_input();

        if choose == String::from("1") {
            if let Err(e) = exec::run(&Formula::new_list(), &mut user) {
                println!("Application error: {}", e);
                process::exit(1);
            }
        } else if choose == String::from("2") {
            utils::print_profile(&user);
        } else if choose == String::from("0") {
            println!("再见!");
            process::exit(1);
        } else {
            println!("请输入正确的选项!");
            continue;
        }
    }
}
