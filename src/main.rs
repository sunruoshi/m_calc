mod exec;
mod lib;

use lib::{Formula, User};
use std::{env, process};

fn main() {
    println!("我的口算 v0.1.0");

    let args: Vec<String> = env::args().collect();

    let user = User::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("欢迎, {}", user.username);

    if let Err(e) = exec::run(&Formula::new_list(), user) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
