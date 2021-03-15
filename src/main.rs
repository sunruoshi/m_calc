mod lib;

use console::style;
use lib::User;
use std::{env, process};

fn main() {
    process::Command::new("clear").status().unwrap();

    println!("{}", style("\n我的口算 v0.3.2").cyan().bold());

    let mut user: User = User::new(env::args()).unwrap_or_else(|e| {
        println!("Problem parsing argument: {}", style(e).red());
        process::exit(1);
    });

    println!(
        "\n{} {}\n",
        style("欢迎,").cyan().bold(),
        style(&user.username).yellow().underlined()
    );

    user.gen_record().printstd();

    print!("\n");

    loop {
        user.select().expect("crates error");
    }
}
