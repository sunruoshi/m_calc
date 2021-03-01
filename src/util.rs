pub mod formula {
    pub struct Formula {
        pub index: u32,
        pub operator: u32,
        pub num1: u32,
        pub num2: u32,
    }

    impl Formula {
        pub fn get_answer(&self) -> u32 {
            return if self.operator == 0 {
                self.num1 + self.num2
            } else {
                self.num1 - self.num2
            };
        }
        pub fn get_formula(&self) -> String {
            return if self.operator == 0 {
                format!("({}) {} + {} = ( ) ", self.index, self.num1, self.num2)
            } else {
                format!("({}) {} - {} = ( ) ", self.index, self.num1, self.num2)
            };
        }
        pub fn validate(&mut self) {
            if self.operator == 1 && self.num1 < self.num2 {
                self.num1 ^= self.num2;
                self.num2 ^= self.num1;
                self.num1 ^= self.num2;
            }
        }
    }
}

pub mod utils {
    pub fn input_number(low: u32, high: u32) -> u32 {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Some error occurred");
            let valid: u32 = match input.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("请输入数字!");
                    continue;
                }
            };
            if valid < low || valid > high {
                println!("输入范围内的数字:{} - {}", low, high);
                continue;
            } else {
                return valid;
            }
        }
    }
    pub fn input_char() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Some error occurred");
        input.trim().to_string()
    }
    pub fn get_time(time: u32) -> (u32, u32) {
        (time / 60, time % 60)
    }
}
