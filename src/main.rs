use rand::Rng;
use std::io;

fn main() {
    println!("我的口算");

    const MAX_COUNT: u32 = 5;

    let mut count: u32 = 0;
    let mut score: u32 = 0;

    let mut failed = Vec::new();

    loop {
        let num1 = rand::thread_rng().gen_range(10..20);
        let num2 = rand::thread_rng().gen_range(1..10);

        println!("({}) {} + {} = __ ", count + 1, num1, num2);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("请输入数字!");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess == num1 + num2 {
            score += 1;
        } else {
            failed.push([num1, num2]);
        }

        count += 1;

        if count == MAX_COUNT {
            println!("你的得分: {}", score);
            break;
        }
    }
    if score != MAX_COUNT {
        println!("错题: {}", failed.len());
        for [num1, num2] in &failed {
            println!("{} + {} = __", num1, num2);
        }

        println!("是否订正? (y/n)");

        let mut choose = String::new();

        io::stdin().read_line(&mut choose).expect("请输入y或n!");

        if choose.trim() == "y" {
            for [num1, num2] in &failed {
                println!("{} + {} = __", num1, num2);

                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("请输入数字!");

                let valid: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if valid == num1 + num2 {
                    println!("回答正确!");
                } else {
                    println!("回答错误!");
                }
            }
        } else {
            println!("程序已关闭")
        }
    }
}
