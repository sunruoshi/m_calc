mod lib;
mod exec;

use lib::{Config, Formula};
use std::{env, process};

fn main() {
    println!("我的口算 v0.1.0");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("欢迎, {}", config.user);

    let list = Formula::new_list();

    if let Err(e) = exec::run(&list, config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
