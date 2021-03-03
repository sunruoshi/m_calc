mod exec;
mod lib;

use lib::{utils, Formula, User};
use std::{env, process};

fn main() {
    println!("我的口算 v0.1.0");

    let args: Vec<String> = env::args().collect();

    let user = User::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("欢迎, {}\n请选择:\n1->开始做题 2->查看记录", user.username);

    let choose: String = utils::read_input();

    if choose == String::from("1") {
        if let Err(e) = exec::run(&Formula::new_list(), user) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    } else if choose == String::from("2") {
        utils::print_profile(user);
    } else {
        println!("再见!");
        process::exit(1);
    }
}
