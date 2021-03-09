mod lib;

use console::style;
use lib::User;
use std::{env, process};

fn main() {
    println!("{}", style("\n我的口算 v0.2.1").cyan().bold());

    let mut user: User = User::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", style(err).red());
        process::exit(1);
    });

    println!(
        "\n{} {}\n",
        style("欢迎,").cyan().bold(),
        style(&user.username).yellow().underlined()
    );

    loop {
        user.select().expect("crates error");
    }
}
